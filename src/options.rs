use std::{
    fs::{read_to_string, File},
    io::Write,
    path::PathBuf,
};

use etcetera::{choose_app_strategy, AppStrategy, AppStrategyArgs};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Options {
    executable_path: String,
}

impl Options {
    pub fn load() -> Self {
        if let Ok(config) = read_to_string(Self::config_path()) {
            toml::from_str(&config).unwrap()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) {
        let mut file = File::create(Self::config_path()).unwrap();
        write!(file, "{}", toml::to_string_pretty(self).unwrap()).unwrap();
    }

    fn config_path() -> PathBuf {
        choose_app_strategy(AppStrategyArgs {
            top_level_domain: "org".to_owned(),
            author: "Salsa Gal".to_owned(),
            app_name: "manifest".to_owned(),
        })
        .map(|strategy| strategy.config_dir())
        .unwrap_or_default()
    }
}
