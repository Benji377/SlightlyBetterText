use std::fs::{self, File};
use std::path::PathBuf;
use iced::highlighter;
use serde::{Deserialize, Serialize};
use directories::{ProjectDirs, UserDirs};
use std::io::Read;

use crate::SETTINGS_FILE_NAME;

// TODO: Add a way to change the hotkey to open the app

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    #[serde(skip)]
    file_path: Option<PathBuf>,
    pub startup_file_path: PathBuf,
    theme: String,
    pub word_wrap: bool,
}

impl Default for Settings {
    fn default() -> Self {
        let user_dirs = UserDirs::new().expect("Failed to get user directories");
        let document_dir_pathbuf = user_dirs
            .document_dir()
            .map(|dir| dir.to_owned().join("sbt_notes.txt"))
            .expect("Failed to get document directory");

        // If the file doesn't exist, create it
        if !document_dir_pathbuf.exists() {
            log::info!("Settings file does not exist, creating it");
            let _ = std::fs::File::create(&document_dir_pathbuf);
        }

        Settings {
            file_path: None,
            startup_file_path: document_dir_pathbuf,
            theme: "solarized".to_owned(),
            word_wrap: true,
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

        log::info!("Loading settings from: {:?}", path);

        // Now load the settings file
        let mut file = File::open(path).map_err(|error| format!("Failed to open settings file: {error}")).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|error| format!("Failed to read settings file: {error}")).unwrap();
        let mut settings_from_str: Settings = serde_json::from_str(&contents).map_err(|error| format!("Failed to deserialize settings: {error}")).unwrap();
        settings_from_str.set_path().unwrap();
        log::info!("Loaded settings: {:?}", settings_from_str);

        Ok(settings_from_str)
    }
    pub fn get_theme(&self) -> highlighter::Theme {
        match self.theme.to_lowercase().as_str() {
            "eighties" => highlighter::Theme::Base16Eighties,
            "mocha" => highlighter::Theme::Base16Mocha,
            "ocean" => highlighter::Theme::Base16Ocean,
            "github" | "inspired github" => highlighter::Theme::InspiredGitHub,
            "solarized" | "solarized dark" => highlighter::Theme::SolarizedDark,
            _ => highlighter::Theme::SolarizedDark,
        }
    }
}