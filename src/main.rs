use tracing::{error, info, instrument, Level};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

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

    start_tracing();

}
