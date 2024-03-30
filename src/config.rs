use serde::{Serialize, Deserialize};
use color_eyre::{eyre::eyre, Result, eyre::WrapErr};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    // pub path: String,
    pub retry_count: u8,
    pub worker_count: u8,
}

pub fn load_config() -> Result<Config> {
    let config = toml::from_str::<Config>(include_str!("../config/config.toml"))
        .wrap_err_with(|| eyre!("Failed to read config from config.toml"))?;
    Ok(config)
}

pub fn get_worker_count() -> u8 {
    load_config().unwrap().worker_count
}

pub fn get_retry_count() -> u8 {
    load_config().unwrap().retry_count
}

// pub fn get_base_dir() -> String {
//     load_config().unwrap().path
// }