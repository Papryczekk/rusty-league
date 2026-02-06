use serde::{Deserialize, Serialize};
use std::fs;
use directories::ProjectDirs;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Account {
    pub username: String,
    pub password: String,
    pub region: String,
    pub in_game_name: String,
    pub custom_tag: String,
}

impl Account {
    pub fn new(
        username: String,
        password: String,
        region: String,
        in_game_name: String,
        custom_tag: String,
    ) -> Self {
        Self {
            username,
            password,
            region,
            in_game_name,
            custom_tag,
        }
    }

    pub fn full_name(&self) -> String {
        if self.custom_tag.trim().is_empty() {
            format!("{}#{}", self.in_game_name, self.region)
        } else {
            format!("{}#{}", self.in_game_name, self.custom_tag)
        }
    }
}

pub fn save_accounts(accounts: &[Account]) -> std::io::Result<()> {
    if let Some(proj_dirs) = ProjectDirs::from("pl", "Rusty Credentials", "") {
        let config_dir = proj_dirs.config_dir();
        if !config_dir.exists() {
            fs::create_dir_all(config_dir)?;
        }
        
        let file_path = config_dir.join("credentials.json");
        let json = serde_json::to_string_pretty(accounts)?;
        fs::write(file_path, json)?;
        Ok(())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Could not determine config directory"))
    }
}

pub fn load_accounts() -> Vec<Account> {
    if let Some(proj_dirs) = ProjectDirs::from("pl", "Rusty Credentials", "") {
        let file_path = proj_dirs.config_dir().join("credentials.json");
        if !file_path.exists() {
            return Vec::new();
        }

        match fs::read_to_string(file_path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => Vec::new(),
        }
    } else {
        Vec::new()
    }
}
