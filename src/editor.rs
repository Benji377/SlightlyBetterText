//! The editor is an iced application. This module controls the entire logic
//! of the application. It includes both the iced frontend and the logic to manage it.

use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use iced::futures::SinkExt;
use iced::futures::Stream;
use iced::stream;
use iced::Subscription;
use iced::keyboard;
use iced::window;
use iced::widget::{
    self, button, column, container, horizontal_space, row, text,
    text_editor, tooltip,
};
use iced::{Center, Element, Fill, Font, Task, Theme};
use std::ffi;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::settings;
use crate:: START_KEY;

/// An error that can occur while interacting with the editor
#[derive(Debug, Clone)]
pub enum Error {
    /// The dialog was closed
    DialogClosed,
    /// An I/O error occurred
    #[allow(dead_code)]
    IoError(io::ErrorKind),
}

/// The editor of the application
pub struct Editor {
    /// The Iced window ID, needed to execute tasks
    window_id: Option<iced::window::Id>,
    /// The file currently being edited
    file: Option<PathBuf>,
    /// The content of the editor
    content: text_editor::Content,
    /// Whether a file is currently being loaded
    is_loading: bool,
    /// Whether the file has been modified
    is_dirty: bool,
    /// Whether the window is visible
    is_visible: bool,
    /// The global hotkey manager
    _key_manager: GlobalHotKeyManager,
    /// The application settings
    _settings: settings::Settings,
}

/// The messages that can be sent to the editor
#[derive(Debug, Clone)]
pub enum Message {
    /// An action was performed in the editor
    ActionPerformed(text_editor::Action),
    /// Create a new file
    NewFile,
    /// Open a file
    OpenFile,
    /// A file was opened
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    /// Save the file
    SaveFile,
    /// A file was saved
    FileSaved(Result<PathBuf, Error>),
    /// A hotkey was pressed
    HotkeyPressed(GlobalHotKeyEvent),
    /// Initialize the window and set the window ID (only called once)
    InitWindow(Option<iced::window::Id>),
}

impl Editor {
    /// Create a new editor instance. WE regsiter the global hotkey manager, the settings module and the window ID here.
    pub fn new() -> (Self, Task<Message>) {
        // Registers hotkey for the app
        let hotkey_manager = GlobalHotKeyManager::new().expect("Failed to create hotkey manager");
        hotkey_manager.register(*START_KEY).expect("Failed to register hotkey");

        // Load settings and get the default file path
        let app_settings = settings::Settings::new().expect("Failed to load settings");
        let default_file = app_settings.startup_file_path.clone();

        (
            Self {
                window_id: None,
                file: None,
                content: text_editor::Content::new(),
                is_loading: true,
                is_dirty: false,
                is_visible: true,
                _key_manager: hotkey_manager,
                _settings: app_settings,
            },
            Task::batch([
                // Load the default file
                Task::perform(
                    load_file(default_file),
                    Message::FileOpened,
                ),
                // Get the window ID
                iced::window::get_latest().map(Message::InitWindow),
                widget::focus_next(),

            ]),
        )
    }

    /// Internal iced update cycle
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ActionPerformed(action) => {
                self.is_dirty = self.is_dirty || action.is_edit();

                self.content.perform(action);

                Task::none()
            }
            Message::NewFile => {
                if !self.is_loading {
                    self.file = None;
                    self.content = text_editor::Content::new();
                }

                Task::none()
            }
            Message::OpenFile => {
                if self.is_loading {
                    Task::none()
                } else {
                    self.is_loading = true;

                    Task::perform(open_file(), Message::FileOpened)
                }
            }
            Message::FileOpened(result) => {
                self.is_loading = false;
                self.is_dirty = false;

                if let Ok((path, contents)) = result {
                    self.file = Some(path);
                    self.content = text_editor::Content::with_text(&contents);
                }

                Task::none()
            }
            Message::SaveFile => {
                if self.is_loading {
                    Task::none()
                } else {
                    self.is_loading = true;

                    Task::perform(
                        save_file(self.file.clone(), self.content.text()),
                        Message::FileSaved,
                    )
                }
            }
            Message::FileSaved(result) => {
                self.is_loading = false;

                if let Ok(path) = result {
                    self.file = Some(path);
                    self.is_dirty = false;
                }

                Task::none()
            }
            Message::HotkeyPressed(hotkey) => {
                if hotkey.state == HotKeyState::Released {
                    let window_id = self.window_id.expect("Window ID not set");
                    if hotkey.id == START_KEY.id {
                        self.is_visible = !self.is_visible;
                        if self.is_visible {
                            log::info!("Showing window");
                            return iced::window::change_mode(window_id, window::Mode::Windowed);
                        } else {
                            log::info!("Hiding window");
                            return iced::window::change_mode(window_id, window::Mode::Hidden);
                        }
                    } else {
                        log::info!("Unknown hotkey event: {:?}", hotkey);
                    }
                }
                Task::none()
            }
            Message::InitWindow(id) => {
                self.window_id = id;
                Task::none()
            }
        }
    }

    /// Internal iced view cycle
    pub fn view(&self) -> Element<Message> {
        let controls = row![
            action(new_icon(), "New file", Some(Message::NewFile)),
            action(
                open_icon(),
                "Open file",
                (!self.is_loading).then_some(Message::OpenFile)
            ),
            action(
                save_icon(),
                "Save file",
                self.is_dirty.then_some(Message::SaveFile)
            ),
            horizontal_space()
        ]
        .spacing(10)
        .align_y(Center);

        let status = row![
            text(if let Some(path) = &self.file {
                let path = path.display().to_string();

                if path.len() > 60 {
                    format!("...{}", &path[path.len() - 40..])
                } else {
                    path
                }
            } else {
                String::from("New file")
            }),
            horizontal_space(),
            text({
                let (line, column) = self.content.cursor_position();

                format!("{}:{}", line + 1, column + 1)
            })
        ]
        .spacing(10);

        column![
            controls,
            text_editor(&self.content)
                .height(Fill)
                .on_action(Message::ActionPerformed)
                .wrapping(if self._settings.word_wrap {
                    text::Wrapping::Word
                } else {
                    text::Wrapping::None
                })
                .highlight(
                    self.file
                        .as_deref()
                        .and_then(Path::extension)
                        .and_then(ffi::OsStr::to_str)
                        .unwrap_or("txt"),
                    self._settings.get_theme(),
                )
                .key_binding(|key_press| {
                    match key_press.key.as_ref() {
                        keyboard::Key::Character("s")
                            if key_press.modifiers.command() =>
                        {
                            log::info!("Save file");
                            Some(text_editor::Binding::Custom(
                                Message::SaveFile,
                            ))
                        }
                        keyboard::Key::Character("o")
                            if key_press.modifiers.command() =>
                        {
                            log::info!("Open file");
                            Some(text_editor::Binding::Custom(
                                Message::OpenFile,
                            ))
                        }
                        keyboard::Key::Character("n")
                            if key_press.modifiers.command() =>
                        {
                            log::info!("New file");
                            Some(text_editor::Binding::Custom(
                                Message::NewFile,
                            ))
                        }
                        _ => text_editor::Binding::from_key_press(key_press),
                    }
                }),
            status,
        ]
        .spacing(10)
        .padding(10)
        .into()
    }

    /// Internal iced theme selection
    pub fn theme(&self) -> Theme {
        if self._settings.get_theme().is_dark() {
            Theme::Dark
        } else {
            Theme::Light
        }
    }

    /// Internal iced subscription cycle
    pub fn subscription(&self) -> iced::Subscription<Message> {
        // Subscribe to hotkey events
        Subscription::run(hotkey_worker)
    }
}
/// Opens a file dialog to select a file to open
async fn open_file() -> Result<(PathBuf, Arc<String>), Error> {
    let picked_file = rfd::AsyncFileDialog::new()
        .set_title("Open a text file...")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    load_file(picked_file).await
}

/// Asynchronously loads a file from the file system
async fn load_file(
    path: impl Into<PathBuf>,
) -> Result<(PathBuf, Arc<String>), Error> {
    let path = path.into();

    let contents = tokio::fs::read_to_string(&path)
        .await
        .map(Arc::new)
        .map_err(|error| Error::IoError(error.kind()))?;

    Ok((path, contents))
}

/// Asynchronously saves a file to the file system
async fn save_file(
    path: Option<PathBuf>,
    contents: String,
) -> Result<PathBuf, Error> {
    let path = if let Some(path) = path {
        path
    } else {
        rfd::AsyncFileDialog::new()
            .save_file()
            .await
            .as_ref()
            .map(rfd::FileHandle::path)
            .map(Path::to_owned)
            .ok_or(Error::DialogClosed)?
    };

    tokio::fs::write(&path, contents)
        .await
        .map_err(|error| Error::IoError(error.kind()))?;

    Ok(path)
}

/// Creates an action button
fn action<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    label: &'a str,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let action = button(container(content).center_x(30));

    if let Some(on_press) = on_press {
        tooltip(
            action.on_press(on_press),
            label,
            tooltip::Position::FollowCursor,
        )
        .style(container::rounded_box)
        .into()
    } else {
        action.style(button::secondary).into()
    }
}

/// Icon for the "new" action
fn new_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e800}')
}

/// Icon for the "save" action
fn save_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e801}')
}

/// Icon for the "open" action
fn open_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0f115}')
}

/// Creates an icon element
fn icon<'a, Message>(codepoint: char) -> Element<'a, Message> {
    const ICON_FONT: Font = Font::with_name("editor-icons");

    text(codepoint).font(ICON_FONT).into()
}

/// Worker that listens for global hotkey events
fn hotkey_worker() -> impl Stream<Item = Message> {
    stream::channel(100, |mut sender| async move {
        // Create channel
        let receiver = GlobalHotKeyEvent::receiver();
        // poll for global hotkey events every 50ms
        loop {
            if let Ok(event) = receiver.try_recv() {
                sender
                    .send(Message::HotkeyPressed(event))
                    .await
                    .unwrap();
                }
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            }
    })
}