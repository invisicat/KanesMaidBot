use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigurationData {
    pub bot: BotConfig,
    pub api: ApiConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BotConfig {
    pub logging: LoggingConfig,
    pub discord: DiscordConfig,
}

/*
BOT RELATED
 */
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DiscordConfig {
    pub app_id: u64,
    pub client_id: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoggingConfig {
    pub enabled: bool,
    pub level: String,
}

/*
API RELATED
 */
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiConfig {
    pub influx: InfluxConfig,
    pub redis: RedisConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InfluxConfig {
    pub enabled: bool,
    pub url: String,
    pub db: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RedisConfig {
    pub enabled: bool,
}
