use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

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
    let dir_path = "credentials";
    if !Path::new(dir_path).exists() {
        fs::create_dir(dir_path)?;
    }
    
    let file_path = format!("{}/credentials.json", dir_path);
    let json = serde_json::to_string_pretty(accounts)?;
    fs::write(file_path, json)?;
    Ok(())
}

pub fn load_accounts() -> Vec<Account> {
    let file_path = "credentials/credentials.json";
    if !Path::new(file_path).exists() {
        return Vec::new();
    }

    match fs::read_to_string(file_path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}
