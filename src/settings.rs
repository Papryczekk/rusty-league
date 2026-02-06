use serde::{Deserialize, Serialize};
use std::fs;
use directories::ProjectDirs;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
    pub riot_client_path: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            riot_client_path: String::new(),
        }
    }
}

pub fn save_settings(settings: &Settings) -> std::io::Result<()> {
    if let Some(proj_dirs) = ProjectDirs::from("pl", "Rusty Credentials", "") {
        let config_dir = proj_dirs.config_dir();
        if !config_dir.exists() {
            fs::create_dir_all(config_dir)?;
        }
        
        let file_path = config_dir.join("settings.json");
        let json = serde_json::to_string_pretty(settings)?;
        fs::write(file_path, json)?;
        Ok(())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Could not determine config directory"))
    }
}

pub fn load_settings() -> Settings {
    if let Some(proj_dirs) = ProjectDirs::from("pl", "Rusty Credentials", "") {
        let file_path = proj_dirs.config_dir().join("settings.json");
        if !file_path.exists() {
            return Settings::default();
        }

        match fs::read_to_string(file_path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => Settings::default(),
        }
    } else {
        Settings::default()
    }
}
