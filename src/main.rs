pub(crate) mod boosty_api;
mod commands;
pub(crate) mod translations;
pub(crate) mod utils;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use crate::{
    boosty_api::{auth::AuthData, BoostyClientBuilder},
    commands::{handle_command, handle_unknown_command, Command},
    translations::load_langs,
    utils::Bot,
};
use std::sync::Arc;
use teloxide::{prelude::*, types::ParseMode, Bot as TeloxideBot};

async fn skip_updates(bot: &Bot) {
    let old_updates = bot
        .get_updates()
        .limit(1)
        .offset(-1)
        .await
        .expect("Unable to get the latest update.");

    if let Some(latest_update) = old_updates.last() {
        bot.get_updates()
            .offset(latest_update.id + 1)
            .await
            .expect("Unable to skip updates.");

        info!("Successfully skipped old updates.");
    }
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info");

    if dotenvy::dotenv().is_err() {
        warn!(".env file not found or failed to read.")
    };

    pretty_env_logger::init();
    info!("Starting bot...");

    let bot = TeloxideBot::from_env().parse_mode(ParseMode::Html);

    skip_updates(&bot).await;

    let lang_bundle = Arc::new(load_langs().await);
    let boosty_client = BoostyClientBuilder::new(AuthData::load()).build();

    let handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<Command>()
                .endpoint(handle_command),
        )
        .branch(dptree::endpoint(handle_unknown_command));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![lang_bundle, boosty_client])
        .default_handler(|upd| async move {
            log::warn!("Unhandled update: {:?}", upd);
        })
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error has occurred in the dispatcher",
        ))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
