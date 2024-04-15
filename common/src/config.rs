//
//  Copyright 2024 Ram Flux, LLC.
//

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Config {
    pub http: Http,
    pub log_level: String,
    pub log_write: bool,
    pub log_dir_name: String,
    pub redis_uri: String,
    pub redis_pool_size: u32,
    pub db_uri: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Http {
    pub port: u16,
}

impl Config {
    pub fn init<P: AsRef<std::path::Path>>(path: P) -> Config {
        let config_text = std::fs::read_to_string(path).unwrap();
        toml::from_str(&config_text).unwrap()
    }
}

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    // config path
    #[arg(short, long)]
    pub config: Option<String>,
}
