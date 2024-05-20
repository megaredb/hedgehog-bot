use std::sync::Arc;

use crate::{
    boosty_api::{types::subscribers::SearchRequest, BoostyClient},
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
            let value = boosty_client
                .search(&SearchRequest {
                    chunk: email.clone(),
                })
                .await;

            let pattern_id = match value {
                Ok(result)
                    if (result.data.len() == 1 && result.data.last().unwrap().email == email) =>
                {
                    "user-found"
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
