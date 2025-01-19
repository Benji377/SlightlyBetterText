// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use iced::Font;
use log::LevelFilter;
use simplelog::TermLogger;
use std::sync::LazyLock;

mod editor;
mod settings;
use editor::Editor;


static APP_NAME: &str = "SlightlyBetterText";
static SETTINGS_FILE_NAME: &str = "settings.json";
static LOGO: &[u8] = include_bytes!("assets/images/logo.ico");
static START_KEY: LazyLock<HotKey> = LazyLock::new(|| HotKey { id: 19012025, key: Code::Space, mods: Modifiers::CONTROL | Modifiers::ALT });

#[cfg(debug_assertions)]
static LOGGING_FILTER: LevelFilter = LevelFilter::Debug;

#[cfg(not(debug_assertions))]
static LOGGING_FILTER: LevelFilter = LevelFilter::Info;


pub fn main() -> iced::Result {
    let log_config = simplelog::ConfigBuilder::new()
        .add_filter_ignore_str("wgpu_core")
        .add_filter_ignore_str("cosmic_text")
        .add_filter_ignore_str("naga")
        .build();

    TermLogger::init(
        LOGGING_FILTER,
        log_config,
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    ).expect("Failed to initialize logger");

    let window_settings = iced::window::Settings {
        visible: true,
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