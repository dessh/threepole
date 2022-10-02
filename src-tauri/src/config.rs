use crate::consts::APP_NAME;
use anyhow::{anyhow, Result};
use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs::{create_dir_all, read_to_string};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub account_platform: usize,
    pub account_id: String,
    pub display_name: String,
    pub display_tag: usize,
}

impl Config {
    pub async fn load() -> Result<Self> {
        let str = read_to_string(Self::get_path()?).await?;
        Ok(serde_json::from_str::<Config>(&str)?)
    }

    pub async fn write(&self) -> Result<()> {
        let path = Self::get_path()?;
        let mut dir = path.clone();
        dir.pop();

        create_dir_all(dir).await?;

        Ok(std::fs::write(path, serde_json::to_string(&self)?)?)
    }

    fn get_path() -> Result<PathBuf> {
        BaseDirs::new()
            .map(|d| {
                let mut path = d.data_dir().to_owned();
                path.push(APP_NAME);
                path.push("config.json");
                path
            })
            .ok_or(anyhow!("no config path for os"))
    }
}
