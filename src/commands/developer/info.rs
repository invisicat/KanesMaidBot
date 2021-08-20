use serenity::prelude::Context;
use serenity::model::prelude::Message;
use serenity::framework::standard::{macros::*, CommandResult};
use crate::utils;


#[command]
#[description = "Info about Kane's Maid"]
async fn info(ctx: &Context, message: &Message) -> CommandResult {
    utils::message::reply(&ctx.http, message, "Check out the Github at https://github.com/RiceCX/KanesMaidBot").await
}