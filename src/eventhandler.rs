use crate::events::{ready};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::channel::Message,
    model::{gateway::Ready, guild::Guild}
};
use serenity::model::id::GuildId;

pub struct Handler;


#[async_trait]
impl EventHandler for Handler {
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
    //    cache_ready(ctx, _guilds).await;
    }
    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: bool) {
    //    guild_create(ctx,guild,is_new).await;
    }
    async fn message(&self, ctx: Context, msg: Message) {
    //    message::message(ctx, msg).await;
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        ready::ready(ctx, ready).await;
    }
}