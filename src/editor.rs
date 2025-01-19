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

#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    #[allow(dead_code)]
    IoError(io::ErrorKind),
}

pub struct Editor {
    window_id: Option<iced::window::Id>,
    file: Option<PathBuf>,
    content: text_editor::Content,
    is_loading: bool,
    is_dirty: bool,
    is_visible: bool,
    _key_manager: GlobalHotKeyManager,
    _settings: settings::Settings,
}

#[derive(Debug, Clone)]
pub enum Message {
    ActionPerformed(text_editor::Action),
    NewFile,
    OpenFile,
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    SaveFile,
    FileSaved(Result<PathBuf, Error>),
    HotkeyPressed(GlobalHotKeyEvent),
    InitWindow(Option<iced::window::Id>),
}

impl Editor {
    pub fn new() -> (Self, Task<Message>) {
        // Registers hotkey for the app
        let hotkey_manager = GlobalHotKeyManager::new().expect("Failed to create hotkey manager");
        hotkey_manager.register(*START_KEY).expect("Failed to register hotkey");

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
                Task::perform(
                    load_file(default_file),
                    Message::FileOpened,
                ),
                iced::window::get_latest().map(Message::InitWindow),
                widget::focus_next(),

            ]),
        )
    }

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

    pub fn theme(&self) -> Theme {
        if self._settings.get_theme().is_dark() {
            Theme::Dark
        } else {
            Theme::Light
        }
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        Subscription::run(hotkey_worker)
    }
}

async fn open_file() -> Result<(PathBuf, Arc<String>), Error> {
    let picked_file = rfd::AsyncFileDialog::new()
        .set_title("Open a text file...")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    load_file(picked_file).await
}

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

fn new_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e800}')
}

fn save_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e801}')
}

fn open_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0f115}')
}

fn icon<'a, Message>(codepoint: char) -> Element<'a, Message> {
    const ICON_FONT: Font = Font::with_name("editor-icons");

    text(codepoint).font(ICON_FONT).into()
}

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