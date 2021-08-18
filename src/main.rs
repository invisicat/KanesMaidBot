use crate::config::ConfigurationData;
use crate::data::{ConfigContainer, ReqwestContainer, ShardManagerContainer, UptimeContainer};
use crate::eventhandler::Handler;
use crate::utils::{read_config, AppInfo};
use chrono::Utc;
use dotenv::dotenv;
use reqwest::redirect::Policy;
use reqwest::Client as ReqwestClient;
use serenity::client::bridge::gateway::GatewayIntents;
use serenity::client::ClientBuilder;
use serenity::framework::StandardFramework;
use serenity::Client;
use std::sync::Arc;
use tracing::{info, instrument};

#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

mod config;
mod constants;
mod data;
mod eventhandler;
mod events;
mod utils;

#[tokio::main(worker_threads = 16)]
#[instrument]
async fn main() {
    print!("\x1B[2J\x1B[1;1H");
    println!("{}\n\n", constants::TITLE);

    dotenv().ok();

    let config = read_config("config.toml");
    let app_id = config.bot.discord.app_id;
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a discord token in env");

    if config.bot.logging.enabled {
        utils::start_logging(&config.bot.logging.level);
    }
    let app_info = utils::get_owners(&token).await;

    let framework: StandardFramework = build_framework(app_info);

    let mut client = ClientBuilder::new(&token)
        .event_handler(Handler)
        .application_id(app_id)
        .intents(GatewayIntents::all())
        .framework(framework)
        .await
        .expect("Could not create client");

    build_client_data(&client, &config).await;

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl + c handler");
        info!("Shutting down!");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start_autosharded().await {
        panic!("An error occurred while running the client: {:?}", why);
    }
}

fn build_framework(app_info: AppInfo) -> StandardFramework {
    StandardFramework::new().configure(|config| {
        config.on_mention(Some(app_info.bot_id));
        config.ignore_bots(true);
        config.ignore_webhooks(true);
        config.case_insensitivity(true);
        config.no_dm_prefix(true);
        config.owners(app_info.owners);

        config
    })
}

async fn build_client_data(client: &Client, config: &ConfigurationData) {
    {
        // Lock so we can edit!
        let mut data = client.data.write().await;

        let http_client = ReqwestClient::builder()
            .user_agent(constants::REQWEST_USER_AGENT)
            .redirect(Policy::none())
            .build()
            .expect("Could not build reqwest client");

        data.insert::<ReqwestContainer>(http_client);
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<ConfigContainer>(config.clone());
        data.insert::<UptimeContainer>(Utc::now());
    }
}
