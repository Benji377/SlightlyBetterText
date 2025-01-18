use iced::Font;
use lazy_static::lazy_static;
use std::sync:: Mutex;
use editor::settings::Settings;
use editor::editor::Editor;

mod editor;

static SETTINGS_FILE_NAME: &str = "settings.json";
static APP_NAME: &str = "SlightlyBetterText";

lazy_static! {
    static ref CONFIG: Mutex<Settings> = Mutex::new(Settings::new().expect("Failed to create settings"));
}


pub fn main() -> iced::Result {
    iced::application(APP_NAME, Editor::update, Editor::view)
        .theme(Editor::theme)
        .font(include_bytes!("../assets/fonts/icons.ttf").as_slice())
        .default_font(Font::MONOSPACE)
        .run_with(Editor::new)
}