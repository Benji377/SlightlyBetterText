use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use iced::Font;
use log::LevelFilter;
use simplelog::TermLogger;
use editor::editor::Editor;
mod editor;

static APP_NAME: &str = "SlightlyBetterText";
use std::sync::LazyLock;

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

    let mut window_settings = iced::window::Settings::default();
    window_settings.visible = true;

    iced::application(APP_NAME, Editor::update, Editor::view)
        .subscription(Editor::subscription)
        .theme(Editor::theme)
        .font(include_bytes!("../assets/fonts/icons.ttf").as_slice())
        .default_font(Font::MONOSPACE)
        .centered()
        .window(window_settings)
        .run_with(Editor::new)
}