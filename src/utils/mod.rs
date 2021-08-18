pub mod db_connection;

use crate::config::ConfigurationData;

use serenity::http::Http;
use serenity::model::id::UserId;
use std::collections::HashSet;
use std::{fs::File, io::Read};
use tracing::{error, info, Level};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub fn read_config(file: &str) -> ConfigurationData {
    let mut file = File::open(file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    toml::from_str::<ConfigurationData>(&contents).unwrap()
}

pub fn start_logging(base_level: &str) {
    LogTracer::init().expect("Could not initialize tracer");

    let level = match base_level {
        "error" => Level::ERROR,
        "warn" => Level::WARN,
        "info" => Level::INFO,
        "debug" => Level::DEBUG,
        "trace" => Level::TRACE,
        _ => Level::TRACE,
    };

    let subscriber = FmtSubscriber::builder()
        .with_target(false)
        .with_max_level(level)
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Could not set global subscriber");

    info!("Tracing started with logging level set to {}", level);
}

pub struct AppInfo {
    pub owners: HashSet<UserId>,
    pub bot_id: UserId,
}

pub async fn get_owners(token: &String) -> AppInfo {
    let http = Http::new_with_token(token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            (Some(owners), Some(info.id))
        }
        Err(why) => {
            error!("Unable to retrieve application info {:?}", why);
            (None, None)
        }
    };

    AppInfo {
        owners: owners.expect("Could not get owner map"),
        bot_id: bot_id.expect("Could not get bot id"),
    }
}
