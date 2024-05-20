use teloxide::{adaptors::DefaultParseMode, Bot as TeloxideBot};

pub type Bot = DefaultParseMode<TeloxideBot>;
