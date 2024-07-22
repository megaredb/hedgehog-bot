mod boosty_api;
mod commands;
mod db;
mod handlers;
pub mod models;
pub mod schema;
mod translations;
mod utils;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use std::{env, sync::Arc};

use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use teloxide::{
    dispatching::UpdateFilterExt,
    prelude::*,
    types::{ParseMode, UpdateKind},
    Bot as TeloxideBot,
};
use tokio::time::{sleep, Duration};

use crate::{
    boosty_api::{auth::AuthData, BoostyClient, BoostyClientBuilder},
    commands::{handle_command, handle_unknown_command, Command},
    db::Pool,
    handlers::{chat_join_handler, chat_subscribers_checker},
    translations::load_langs,
    utils::{Bot, Config},
};

async fn skip_updates(bot: &Bot, boosty_client: BoostyClient, pool: Pool, config: Config) {
    let mut last_update_id = -1;

    loop {
        let old_updates = bot
            .get_updates()
            .offset(last_update_id)
            .await
            .expect("Unable to get the updates.");

        if old_updates.is_empty() {
            break;
        }

        last_update_id = (old_updates.last().unwrap().id.0 + 1) as i32;

        for old_update in old_updates {
            if let UpdateKind::ChatJoinRequest(chat_join_request) = &old_update.kind {
                let _ = chat_join_handler(
                    boosty_client.to_owned(),
                    pool.to_owned(),
                    config,
                    bot.to_owned(),
                    chat_join_request.to_owned(),
                )
                .await;
            }
        }
    }

    info!("Successfully skipped old updates.");
}

#[tokio::main]
async fn main() {
    if dotenvy::dotenv().is_err() {
        warn!(".env file not found or failed to read.")
    };

    pretty_env_logger::init();
    info!("Starting bot...");

    let lang_bundle = Arc::new(load_langs().await);
    let boosty_client = BoostyClientBuilder::new(AuthData::load()).build();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let bot = TeloxideBot::from_env().parse_mode(ParseMode::Html);

    let config = Config::build();
    let db_config =
        AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = bb8::Pool::builder().build(db_config).await.unwrap();

    skip_updates(&bot, boosty_client.clone(), pool.clone(), config).await;

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .branch(
                    dptree::entry()
                        .filter_command::<Command>()
                        .endpoint(handle_command),
                )
                .branch(dptree::endpoint(handle_unknown_command)),
        )
        .branch(Update::filter_chat_join_request().endpoint(chat_join_handler));

    let bot_cloned = bot.clone();
    let boosty_cloned = boosty_client.clone();
    let pool_cloned = pool.clone();

    tokio::spawn(async move {
        loop {
            chat_subscribers_checker(
                boosty_cloned.to_owned(),
                pool_cloned.to_owned(),
                config,
                bot_cloned.to_owned(),
            )
            .await
            .unwrap();

            sleep(Duration::from_secs(60 * 60)).await;
        }
    });

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![lang_bundle, boosty_client, pool, config])
        .default_handler(|upd| async move {
            log::warn!("Unhandled update: {:?}", upd);
        })
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
