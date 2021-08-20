use serenity::http::CacheHttp;
use serenity::static_assertions::_core::fmt::Display;
use serenity::framework::standard::{CommandResult, CommandError};
use serenity::model::prelude::*;

pub async fn reply(cache: impl CacheHttp, message: &Message, content: impl Display) -> CommandResult {
    match message.reply(&cache, content).await {
        Ok(_) => {
            Ok(())
        }
        Err(why) => {
            Err(CommandError::from(why))
        }
    }
}