use clap::Parser;
use structopt::StructOpt;
use serde_derive::{Deserialize, Serialize};
// use log::{warn, LevelFilter};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum LogLevel {
   Info,
   Warn,
   Error,
   Debug,
}

impl ToString for LogLevel {
   fn to_string(&self) -> String {
      match *self {
         LogLevel::Error => "error".to_string(),
         LogLevel::Warn => "warn".to_string(),
         LogLevel::Info => "info".to_string(),
         LogLevel::Debug => "debug".to_string(),
      }
   }
}

impl FromStr for LogLevel {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "error" => Ok(LogLevel::Error),
            "warn" => Ok(LogLevel::Warn),
            "info" => Ok(LogLevel::Info),
            "debug" => Ok(LogLevel::Debug),
            _ => Err("Invalid log level"),
        }
    }
}

/// Here is the description of your software.
#[derive(Parser, Debug, StructOpt, Serialize, Deserialize)]
#[command(author, version, about, long_about = None)]
pub struct Args {
   /// The level of log messages to display. Can be one of: error, warn, info, debug.
   #[arg(short, long = "log-level")]
   pub log_level: Option<LogLevel>,

   #[arg(short, long = "remote-peer")]
   pub remote_peer: Option<String>,

   /// The path to the configuration file to be used, with a default value of "./config.json".
   #[arg(short, long = "config", default_value = "./config.json")]
   pub config: String,
}

impl Args {
   pub fn apply_config(&mut self, config: &serde_json::Value) {
      let mut data_map = serde_json::to_value(&self).unwrap_or({
         serde_json::Value::Null
      });

      if let serde_json::Value::Object(ref mut map) = data_map {
         for (key, value) in map.iter_mut() {
            if *value == serde_json::Value::Null {
               let new_value = &config["Args"][key];
               *value = new_value.to_owned();
            }
         }
      }

      let new_args = serde_json::from_value(data_map).unwrap();
      *self = new_args;
   }
}
