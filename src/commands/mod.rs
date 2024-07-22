use std::sync::Arc;

use chrono::{DateTime, Days};
use fluent::FluentArgs;
use regex::Regex;
use teloxide::{dispatching::dialogue::GetChatId, prelude::*, utils::command::BotCommands};

use crate::{
    boosty_api::{
        types::subscribers::{Order, SearchRequest, SortBy, SubscribersRequest},
        BoostyClient,
    },
    db::{
        users::{create_user, get_user, get_user_by_boosty_id, update_user},
        Pool,
    },
    models::User,
    translations::TranslationType,
    utils::Bot,
};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "Поддерживаются следующие команды:"
)]
pub enum Command {
    #[command(description = "запустить бота.")]
    Start,
    #[command(description = "отобразить этот текст.")]
    Help,
    #[command(description = "привязать почту Boosty.")]
    Email(String),
    #[command(description = "просмотреть свой профиль.")]
    Profile,
}

async fn email_command(
    raw_email: String,
    lang_bundle: Arc<TranslationType>,
    boosty_client: BoostyClient,
    pool: Pool,
    bot: Bot,
    msg: Message,
) -> ResponseResult<()> {
    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();

    let mut args = FluentArgs::new();
    let mut errors = vec![];

    let email = match email_regex.find(&raw_email) {
        Some(value) if !value.is_empty() => value.as_str(),
        _ => {
            let text = lang_bundle.format_pattern(
                lang_bundle
                    .get_message("invalid-email")
                    .unwrap()
                    .value()
                    .unwrap(),
                Some(&args),
                &mut errors,
            );

            let _ = bot.send_message(msg.chat_id().unwrap(), text).await;

            return Ok(());
        }
    }
    .to_string();

    let pattern_id = match boosty_client
        .search(&SearchRequest {
            chunk: email.clone(),
        })
        .await
    {
        Ok(result) if (!result.data.is_empty() && result.data.last().unwrap().email == email) => {
            let mut conn = pool.get().await.unwrap();

            let mut pattern = "no-user-found";
            let res = boosty_client
                .subscribers(&SubscribersRequest {
                    user_ids: vec![result.data.last().unwrap().id].into(),
                    sort_by: SortBy::default(),
                    limit: 10,
                    offset: Some(0),
                    order: Order::default(),
                })
                .await;

            if let Ok(user_resp) = res {
                let boosty_user = user_resp.data.last().unwrap();
                let user_resp = get_user(&mut conn, msg.from().unwrap().id.0 as i64).await;

                if get_user_by_boosty_id(&mut conn, boosty_user.basic_info.id as i64)
                    .await
                    .is_ok()
                {
                    pattern = "user-already-exists";
                } else if boosty_user.is_paid() {
                    pattern = "user-found";
                    args.set("name", boosty_user.basic_info.name.clone());
                    args.set("level", boosty_user.level.name.clone());

                    let user_data = User {
                        id: msg.from().unwrap().id.0 as i64,
                        boosty_id: boosty_user.basic_info.id as i64,
                        expires_at: DateTime::from_timestamp(
                            boosty_user.on_time.try_into().unwrap(),
                            0,
                        )
                        .unwrap()
                        .checked_add_days(Days::new(30))
                        .unwrap()
                        .naive_utc(),
                    };

                    if user_resp.is_ok() {
                        update_user(&mut conn, user_data).await.unwrap();
                    } else {
                        create_user(&mut conn, user_data).await.unwrap();
                    }
                } else {
                    pattern = "user-not-subscribed"
                }
            }

            pattern
        }
        Err(err) => {
            error!("{}", err.to_string());
            "no-user-found"
        }
        _ => "no-user-found",
    };

    let pattern = lang_bundle
        .get_message(pattern_id)
        .unwrap_or_else(|| panic!("Message with identifier `{}` doesn't exist.", pattern_id))
        .value()
        .unwrap_or_else(|| panic!("Message with identifier `{}` has empty value.", pattern_id));

    args.set("email", email);

    let text = lang_bundle.format_pattern(pattern, Some(&args), &mut errors);

    bot.send_message(msg.chat.id, text).await?;

    Ok(())
}

async fn profile_command(
    lang_bundle: Arc<TranslationType>,
    boosty_client: BoostyClient,
    pool: Pool,
    bot: Bot,
    msg: Message,
) -> ResponseResult<()> {
    let from_user = msg.from().unwrap();

    let mut conn = pool.get().await.unwrap();
    let user_resp = get_user(&mut conn, from_user.id.0 as i64).await;

    let mut args = FluentArgs::new();
    let mut errors = vec![];

    let pattern_id;

    if let Ok(user) = user_resp {
        let res = boosty_client
            .subscribers(&SubscribersRequest {
                user_ids: vec![user.boosty_id as u64].into(),
                sort_by: SortBy::default(),
                limit: 10,
                offset: Some(0),
                order: Order::default(),
            })
            .await;

        pattern_id = match res {
            Ok(boosty_users)
                if (!boosty_users.data.is_empty()
                    && boosty_users.data.last().unwrap().price > 0.) =>
            {
                let boosty_user = boosty_users.data.last().unwrap();

                args.set("name", boosty_user.basic_info.name.clone());
                args.set("email", boosty_user.basic_info.email.clone());
                args.set("level", boosty_user.level.name.clone());
                args.set("price", boosty_user.level.price);
                args.set(
                    "expires-at",
                    user.expires_at.format("%Y-%m-%d %H:%M:%S").to_string(),
                );

                "profile"
            }
            Ok(_) => "no_profile",
            Err(_) => "profile-api-error",
        };
    } else {
        pattern_id = "no-profile";
    }

    let pattern = lang_bundle
        .get_message(pattern_id)
        .unwrap_or_else(|| panic!("Message with identifier `{}` doesn't exist.", pattern_id))
        .value()
        .unwrap_or_else(|| panic!("Message with identifier `{}` has empty value.", pattern_id));

    let text = lang_bundle.format_pattern(pattern, Some(&args), &mut errors);
    bot.send_message(msg.chat.id, text).await?;

    Ok(())
}

async fn _handle_command(
    lang_bundle: Arc<TranslationType>,
    boosty_client: BoostyClient,
    pool: Pool,
    bot: Bot,
    msg: Message,
    cmd: Option<Command>,
) -> ResponseResult<()> {
    if msg.from().is_none() {
        return Ok(());
    }

    let unpacked_cmd = match cmd {
        Some(value) => value,
        None => Command::Help,
    }
    .clone();

    match unpacked_cmd {
        Command::Start => {
            let pattern = lang_bundle
                .get_message("start")
                .expect("Message with identifier `start` doesn't exist.")
                .value()
                .expect("Message with identifier `start` has no value.");

            let text = lang_bundle
                .format_pattern(pattern, Some(&FluentArgs::new()), &mut vec![])
                .clone();

            bot.send_message(msg.chat.id, text).await?;
        }
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Email(email) => {
            email_command(
                email.trim().to_string(),
                lang_bundle,
                boosty_client,
                pool,
                bot,
                msg,
            )
            .await?;
        }
        Command::Profile => {
            profile_command(lang_bundle, boosty_client, pool, bot, msg).await?;
        }
    };

    Ok(())
}

pub async fn handle_command(
    lang_bundle: Arc<TranslationType>,
    boosty_client: BoostyClient,
    pool: Pool,
    bot: Bot,
    msg: Message,
    cmd: Command,
) -> ResponseResult<()> {
    _handle_command(lang_bundle, boosty_client, pool, bot, msg, Some(cmd)).await
}

pub async fn handle_unknown_command(
    lang_bundle: Arc<TranslationType>,
    boosty_client: BoostyClient,
    pool: Pool,
    bot: Bot,
    msg: Message,
) -> ResponseResult<()> {
    _handle_command(lang_bundle, boosty_client, pool, bot, msg, None).await
}
