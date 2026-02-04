use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

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
    let dir_path = "credentials";
    if !Path::new(dir_path).exists() {
        fs::create_dir(dir_path)?;
    }
    
    let file_path = format!("{}/settings.json", dir_path);
    let json = serde_json::to_string_pretty(settings)?;
    fs::write(file_path, json)?;
    Ok(())
}

pub fn load_settings() -> Settings {
    let file_path = "credentials/settings.json";
    if !Path::new(file_path).exists() {
        return Settings::default();
    }

    match fs::read_to_string(file_path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => Settings::default(),
    }
}
