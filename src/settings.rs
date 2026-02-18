use serde::{Deserialize, Serialize};
use std::fs;
use directories::ProjectDirs;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
    pub riot_client_path: String,
    #[serde(default)]
    pub minimalist_mode: bool,
    #[serde(default)]
    pub start_with_windows: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            riot_client_path: String::new(),
            minimalist_mode: false,
            start_with_windows: false,
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

pub fn set_autostart(enable: bool) -> Result<(), String> {
    use std::process::Command;
    use std::os::windows::process::CommandExt;

    const CREATE_NO_WINDOW: u32 = 0x08000000;
    let app_name = "RustyLeague";

    if enable {
        let exe_path = std::env::current_exe()
            .map_err(|e| format!("Failed to get exe path: {}", e))?
            .to_string_lossy()
            .to_string();

        let output = Command::new("reg")
            .args([
                "add",
                r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run",
                "/v", app_name,
                "/t", "REG_SZ",
                "/d", &format!("\"{}\"" , exe_path),
                "/f",
            ])
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .map_err(|e| format!("Failed to run reg command: {}", e))?;

        if !output.status.success() {
            return Err("Failed to add registry entry".into());
        }
    } else {
        let output = Command::new("reg")
            .args([
                "delete",
                r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run",
                "/v", app_name,
                "/f",
            ])
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .map_err(|e| format!("Failed to run reg command: {}", e))?;

        if !output.status.success() {
        }
    }
    Ok(())
}
