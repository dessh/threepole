use std::{
    fs::{create_dir_all, read_to_string},
    io::ErrorKind,
    path::PathBuf,
};

use anyhow::{anyhow, Result};
use directories::BaseDirs;
use serde::{de::DeserializeOwned, Serialize};

use self::{preferences::Preferences, profiles::Profiles};
use crate::consts::APP_NAME;

pub mod preferences;
pub mod profiles;

pub struct ConfigManager {
    preferences: Preferences,
    profiles: Profiles,
}

impl ConfigManager {
    pub fn load() -> Result<Self> {
        Ok(Self {
            preferences: Preferences::load()?,
            profiles: Profiles::load()?,
        })
    }

    pub fn get_preferences(&self) -> &Preferences {
        &self.preferences
    }

    pub fn get_profiles(&self) -> &Profiles {
        &self.profiles
    }

    pub fn set_preferences(&mut self, preferences: Preferences) -> Result<()> {
        self.preferences = preferences;
        self.preferences.write()
    }

    pub fn set_profiles(&mut self, profiles: Profiles) -> Result<()> {
        self.profiles = profiles;
        self.profiles.write()
    }
}

trait ConfigFile: Serialize + DeserializeOwned + Default {
    fn load() -> Result<Self> {
        match read_to_string(Self::get_path()?) {
            Ok(s) => {
                let def = serde_json::from_str::<Self>(&s)?;
                def.write()?;
                Ok(def)
            }
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    let def = Self::default();
                    def.write()?;
                    Ok(def)
                }
                _ => Err(e.into()),
            },
        }
    }

    fn write(&self) -> Result<()> {
        let path = Self::get_path()?;
        let mut dir = path.clone();
        dir.pop();

        create_dir_all(dir)?;

        Ok(std::fs::write(path, serde_json::to_string(&self)?)?)
    }

    fn get_path() -> Result<PathBuf> {
        BaseDirs::new()
            .map(|d| {
                let mut path = d.data_dir().to_owned();
                path.push(APP_NAME);
                path.push(Self::get_filename());
                path
            })
            .ok_or(anyhow!("No data_dir available"))
    }

    fn get_filename() -> &'static str;
}
