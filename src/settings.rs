use std::fs::{self, File};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use directories::ProjectDirs;
use std::io::Read;

use crate::SETTINGS_FILE_NAME;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    #[serde(skip)]
    file_path: Option<PathBuf>,
    mode: String,
    startup: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            file_path: None,
            mode: "normal".to_string(),
            startup: false,
        }
    }
}

impl Settings {
    fn set_path(&mut self) -> Result<(), String> {
        #[cfg(not(target_os = "windows"))]
        let project_dirs = ProjectDirs::from("sbt", "slightlybettertext", "slightlybettertext").ok_or("Could not find project directories".to_owned()).unwrap();
        #[cfg(target_os = "windows")]
        let project_dirs = ProjectDirs::from("sbt", "slightlybettertext", "").ok_or("Could not find project directories".to_owned()).unwrap();
        let config_dir = project_dirs.config_dir().to_owned();
        fs::create_dir_all(&config_dir).map_err(|error| format!("Failed to create config dir: {error}")).unwrap();
        self.file_path = Some(config_dir);
        Ok(())
    }

    fn save(&self) -> Result<(), String> {
        let file_path = self.file_path.as_ref().ok_or("No file path set".to_owned()).unwrap();
        
        if !file_path.exists() {
            fs::create_dir_all(&file_path).map_err(|error| format!("Failed to create config dir: {error}")).unwrap();
        }

        let file = File::create(file_path.join(SETTINGS_FILE_NAME)).map_err(|error| format!("Failed to create settings file: {error}")).unwrap();

        serde_json::to_writer_pretty(file, self).map_err(|error| format!("Failed to serialize settings: {error}"))
    }

    pub fn new() -> Result<Self, String> {
        let mut tmp = Settings::default();
        tmp.set_path().unwrap();

        let path = tmp.file_path.as_ref().unwrap().join(SETTINGS_FILE_NAME);

        // Check if the settings file exists, else create it
        if !path.exists() {
            tmp.save().unwrap();
        }

        // Now load the settings file
        let mut file = File::open(path).map_err(|error| format!("Failed to open settings file: {error}")).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|error| format!("Failed to read settings file: {error}")).unwrap();
        let mut settings_from_str: Settings = serde_json::from_str(&contents).map_err(|error| format!("Failed to deserialize settings: {error}")).unwrap();
        settings_from_str.set_path().unwrap();
        Ok(settings_from_str)
    }
}