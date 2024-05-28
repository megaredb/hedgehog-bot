use std::sync::Arc;

use crate::{
    boosty_api::{
        types::subscribers::{Order, SearchRequest, SortBy, SubscribersRequest},
        BoostyClient,
    },
    translations::TranslationType,
    utils::Bot,
};
use fluent::FluentArgs;
use teloxide::{prelude::*, utils::command::BotCommands};

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

async fn _handle_command(
    lang_bundle: Arc<TranslationType>,
    boosty_client: BoostyClient,
    bot: Bot,
    msg: Message,
    cmd: Option<Command>,
) -> ResponseResult<()> {
    let unpacked_cmd = match cmd {
        Some(value) => value,
        None => Command::Help,
    };

    let mut args = FluentArgs::new();
    let mut errors = vec![];

    match unpacked_cmd {
        Command::Start => {
            let pattern = lang_bundle
                .get_message("start")
                .expect("Message with identifier `start` doesn't exist.")
                .value()
                .expect("Message with identifier `start` has no value.");

            let text = lang_bundle.format_pattern(pattern, Some(&args), &mut errors);

            bot.send_message(msg.chat.id, text).await?
        }
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Email(email) => {
            let pattern_id = match boosty_client
                .search(&SearchRequest {
                    chunk: email.clone(),
                })
                .await
            {
                Ok(result)
                    if (!result.data.is_empty() && result.data.last().unwrap().email == email) =>
                {
                    let mut pattern = "no-user-found";

                    if let Ok(user_resp) = boosty_client
                        .subscribers(&SubscribersRequest {
                            user_ids: vec![result.data.last().unwrap().id],
                            sort_by: SortBy::default(),
                            limit: 10,
                            offset: Some(0),
                            order: Order::default(),
                        })
                        .await
                    {
                        pattern = "user-found";

                        let user = user_resp.data.last().unwrap();
                        args.set("name", user.basic_info.name.clone());
                        args.set("level", user.level.name.clone());
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
                .expect("Message with identifier `start` doesn't exist.")
                .value()
                .expect("Message with identifier `start` has no value.");

            args.set("email", email);

            let text = lang_bundle.format_pattern(pattern, Some(&args), &mut errors);

            bot.send_message(msg.chat.id, text).await?
        }
        Command::Profile => todo!(),
    };

    Ok(())
}

pub async fn handle_command(
    lang_bundle: Arc<TranslationType>,
    boosty_client: BoostyClient,
    bot: Bot,
    msg: Message,
    cmd: Command,
) -> ResponseResult<()> {
    _handle_command(lang_bundle, boosty_client, bot, msg, Some(cmd)).await
}

pub async fn handle_unknown_command(
    lang_bundle: Arc<TranslationType>,
    boosty_client: BoostyClient,
    bot: Bot,
    msg: Message,
) -> ResponseResult<()> {
    _handle_command(lang_bundle, boosty_client, bot, msg, None).await
}
