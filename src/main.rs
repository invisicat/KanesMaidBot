use tracing::{error, info, instrument, Level};
use crate::utils::read_config;

#[macro_use]
extern crate diesel;


pub mod schema;
pub mod models;

mod constants;
mod config;
mod utils;


#[tokio::main(worker_threads = 16)]
#[instrument]
async fn main() {
    print!("\x1B[2J\x1B[1;1H");
    println!("{}\n\n", constants::TITLE);

    let config = read_config("config.toml");

    if config.bot.logging.enabled {
        utils::start_logging(&config.bot.logging.level);
    }
}

fn build_framework() {

}
