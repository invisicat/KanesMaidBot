use tracing::{error, info, instrument, Level};
use crate::utils::{read_config, AppInfo};
use serenity::framework::StandardFramework;
use std::error::Error;
use serenity::model::prelude::*;
use dotenv::dotenv;
use crate::config::ConfigurationData;
use serenity::client::ClientBuilder;
use serenity::client::bridge::gateway::GatewayIntents;
use serenity::Client;
use reqwest::Client as ReqwestClient;
use reqwest::redirect::Policy;

#[macro_use]
extern crate diesel;


pub mod schema;
pub mod models;

mod constants;
mod config;
mod utils;
mod data;


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

    let framework: StandardFramework = build_framework(app_info, config);

    let mut client = ClientBuilder::new(&token)
        .application_id(app_id)
        .intents(GatewayIntents::all())
        .framework(framework)
        .await?;
}

fn build_framework(app_info: AppInfo, config: ConfigurationData) -> StandardFramework {
    StandardFramework::new()
        .configure(|config| {
            config.on_mention(Some(bot_id));
            config.ignore_bots(true);
            config.ignore_webhooks(true);
            config.case_insensitivity(true);
            config.no_dm_prefix(true);
            config.owners(app_info.owners);

            config
        })
}

async fn build_client_data(client: &Client) {
    {
        // Lock so we can edit!
        let mut data = client.data.write().await;

        let http_client = ReqwestClient::builder().user_agent(constants::REQWEST_USER_AGENT).redirect(Policy::none()).build()?;

    }
}