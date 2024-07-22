use std::env;

use teloxide::{adaptors::DefaultParseMode, Bot as TeloxideBot};

pub type Bot = DefaultParseMode<TeloxideBot>;

#[derive(Clone, Copy)]
pub struct Config {
    pub chat_id: i64,
}

impl Config {
    pub fn build() -> Self {
        let chat_id: i64 = env::var("CHAT_ID")
            .expect("Environment variable `CHAT_ID` is not provided.")
            .parse()
            .expect("Unable to convert enviromnent variable `CHAT_ID` to i64");

        Self { chat_id }
    }
}
