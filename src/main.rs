use std::{path::PathBuf, time::Duration};

use clap::Parser;
use figment::{
    providers::{self, Format},
    Figment,
};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, DurationMilliSeconds};

#[skip_serializing_none]
#[derive(Debug, Parser, Serialize)]
#[command(name = "Example App", author, version, about)]
pub struct Args {
    #[arg(long, short)]
    #[serde(skip_serializing)]
    pub config: Option<PathBuf>,

    #[arg(long, short)]
    pub repetitions: Option<u64>,

    #[arg(long, short)]
    pub interval: Option<u64>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
// #[serde(default)]
pub struct Settings {
    pub repetitions: u64,

    #[serde_as(as = "DurationMilliSeconds")]
    pub interval: Duration,

    pub personal_message: MessageSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageSettings {
    name: String,
}

// impl Default for Settings {
//     fn default() -> Self {
//         Self {
//             repetitions: 1,
//             interval: Duration::from_millis(100),
//         }
//     }
// }

fn configuration() -> Figment {
    let args = Args::parse();

    let figment = Figment::new()
        .join(providers::Serialized::defaults(Args::parse()))
        .join(providers::Env::prefixed("CONFIG_APP__").split("__"));

    if let Some(config) = args.config {
        figment.join(providers::Yaml::file(config))
    } else {
        figment
    }
}

fn main() {
    let config = configuration();

    let settings = config.extract::<Settings>();
    if let Err(error) = settings {
        eprintln!("{error}");
        return;
    }
    let settings = settings.unwrap();

    println!("With settings: {settings:#?}");

    // Run the application logic.
    for times in 0..settings.repetitions {
        println!(
            "Hi {}, for the {times} time",
            settings.personal_message.name
        );
        std::thread::sleep(settings.interval);
    }
}
