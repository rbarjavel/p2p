use std::fs;
use clap::Parser;
use log::{error, info, warn, debug};
use env_logger;
mod args;
mod peer;

use env_logger::Env;

use futures::StreamExt;

#[async_std::main]
async fn main() {
    let mut arg = args::Args::parse();
    println!("{:?}", arg);
    setup_logger(&arg).await;

    let config = &arg.config;
    let config_json = match fs::metadata(config) {
        Ok(_) => read_config(config),
        Err(_) => {
            warn!("No config file found at '{}'", &config);
            "".into()
        },
    };

    arg.apply_config(&config_json);
    match peer::p2p(arg.remote_peer).await {
        Ok(_) => {},
        Err(e) => {
            error!("{:?}", e);
        }
    }
}

async fn setup_logger(arg: &args::Args) {
    println!("=> {:?}", arg);
    let log_level = arg.log_level.unwrap_or({
        args::LogLevel::Info
    });

    env_logger::Builder::from_env(Env::default().default_filter_or(log_level.to_string()))
    .init();
}

fn read_config(config_file: &str) -> serde_json::Value {
    let contents = fs::read_to_string(config_file)
        .expect("Failed to read config file");

    let config = match serde_json::from_str(&contents) {
        Ok(c) => c,
        Err(e) => {
            error!("{}", e);
            error!("{}", &contents);
            serde_json::Value::Null
        }
    };

    config.into()
}
