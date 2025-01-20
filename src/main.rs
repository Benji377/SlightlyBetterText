//! # SlightlyBetterText
//! 
//! SlightlyBetterText (SBT) is a very small and effiecient text editor written fully in Rust.
//! It is heavily inspired by iced.
//! Please check the [GitHub repository](https://github.com/Benji377/SlightlyBetterText) for more information.

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use iced::Font;
use log::LevelFilter;
use simplelog::TermLogger;
use std::sync::LazyLock;

/// The editor itself, as an iced application
mod editor;
/// The settings for the editor
mod settings;
use editor::Editor;

/// The name of the application
static APP_NAME: &str = "SlightlyBetterText";
/// The name of the settings file
static SETTINGS_FILE_NAME: &str = "settings.json";
/// A byte array containing the application icon
static LOGO: &[u8] = include_bytes!("assets/images/logo.ico");
/// The hotkey to start the application
static START_KEY: LazyLock<HotKey> = LazyLock::new(|| HotKey { id: 19012025, key: Code::Space, mods: Modifiers::CONTROL | Modifiers::ALT });

#[cfg(debug_assertions)]
/// Sets the log level to debugging
static LOGGING_FILTER: LevelFilter = LevelFilter::Debug;

#[cfg(not(debug_assertions))]
/// Sets the log level to info
static LOGGING_FILTER: LevelFilter = LevelFilter::Info;

/// The main function. Creates the logger instance and starts the iced app
pub fn main() -> iced::Result {
    // A simple log that logs messages to the CLI, can be extended to also log to files
    let log_config = simplelog::ConfigBuilder::new()
        .add_filter_ignore_str("wgpu_core")
        .add_filter_ignore_str("cosmic_text")
        .add_filter_ignore_str("naga")
        .build();

    if let Err(e) = TermLogger::init(
        LOGGING_FILTER,
        log_config,
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    ) {
        eprintln!("Failed to initialize logger: {}", e);
    }

    let window_settings = iced::window::Settings {
        icon: Some(iced::window::icon::from_file_data(LOGO, None).expect("Failed to load icon")),
        ..iced::window::Settings::default()
    };

    iced::application(APP_NAME, Editor::update, Editor::view)
        .subscription(Editor::subscription)
        .theme(Editor::theme)
        .font(include_bytes!("assets/fonts/icons.ttf").as_slice())
        .default_font(Font::MONOSPACE)
        .centered()
        .window(window_settings)
        .run_with(Editor::new)
}