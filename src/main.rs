use iced::Font;
use editor::editor::Editor;
mod editor;

static APP_NAME: &str = "SlightlyBetterText";


pub fn main() -> iced::Result {
    iced::application(APP_NAME, Editor::update, Editor::view)
        .theme(Editor::theme)
        .font(include_bytes!("../assets/fonts/icons.ttf").as_slice())
        .default_font(Font::MONOSPACE)
        .run_with(Editor::new)
}