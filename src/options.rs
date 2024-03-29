use std::{
    fs::{create_dir_all, read_to_string, File},
    io::Write,
    path::PathBuf,
};

use etcetera::{choose_app_strategy, AppStrategy, AppStrategyArgs};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Options {
    pub dark_theme: bool,
    pub executable_path: String,
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
        create_dir_all(Self::config_path().parent().unwrap()).unwrap();
        let mut file = File::create(Self::config_path()).unwrap();
        write!(file, "{}", toml::to_string_pretty(self).unwrap()).unwrap();
    }

    fn config_path() -> PathBuf {
        choose_app_strategy(AppStrategyArgs {
            top_level_domain: "org".to_owned(),
            author: "Salsa Gal".to_owned(),
            app_name: "manifest".to_owned(),
        })
        .map(|strategy| strategy.config_dir().join("config.toml"))
        .unwrap_or_default()
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            dark_theme: true,
            executable_path: String::new(),
        }
    }
}

pub struct OptionsMenu {
    options: Options,

    pub to_close: bool,
}

impl OptionsMenu {
    pub fn new(options: Options) -> Self {
        Self {
            options,
            to_close: false,
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Options");

            if ui
                .checkbox(&mut self.options.dark_theme, "Dark mode")
                .clicked()
            {
                ctx.set_visuals(match self.options.dark_theme {
                    true => egui::Visuals::dark(),
                    false => egui::Visuals::light(),
                });
            }

            if ui.button("OK").clicked() {
                self.to_close = true;
                self.options.save();
                return;
            }
            self.to_close = ui.button("Cancel").clicked();
        });
    }
}
