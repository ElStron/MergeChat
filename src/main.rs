use iced::widget::{button, center, column, horizontal_space, scrollable, text, text_input};
use iced::{window, Task};
use iced::{Center, Subscription, Theme};
use utils::load_icon;

use std::collections::BTreeMap;
type Renderer = iced::Renderer;
type Element<'a, Message> = iced::Element<'a, Message, Theme, Renderer>;
mod about;
mod chat;
mod settings;
mod utils;

fn main() -> iced::Result {
    iced::daemon(MergeChat::title, MergeChat::update, MergeChat::view)
        .subscription(MergeChat::subscription)
        .theme(MergeChat::theme)
        .scale_factor(MergeChat::scale_factor)
        .run_with(MergeChat::new)
}

struct MergeChat {
    windows: BTreeMap<window::Id, Window>,
    global_twitch_id: String,
    global_youtube_id: String,
}

#[derive(Debug)]
struct Window {
    title: String,
    current_scale: f64,
    theme: Theme,
    view: View,
    previous_view: Option<View>,
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenWindow,
    WindowOpened(window::Id),
    WindowClosed(window::Id),

    AboutPressed(window::Id),
    About(about::AboutMessage),

    ChatPressed(window::Id),
    Chat(chat::ChatMessage),

    OpenSettingsWindow,
    SettingsWindowOpened(window::Id),
    Settings(settings::SettingsMessage),

    TwitchIdChanged(String),
    YoutubeIdChanged(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum View {
    Main,
    About,
    Settings,
    Chat,
}

impl MergeChat {
    fn new() -> (Self, Task<Message>) {
        let (_id, open) = window::open(window::Settings {
            icon: Some(load_icon()),
            transparent: true,
            ..Default::default()
        });

        (
            Self {
                global_twitch_id: String::new(),
                global_youtube_id: String::new(),
                windows: BTreeMap::new(),
            },
            open.map(Message::WindowOpened),
        )
    }

    fn title(&self, window: window::Id) -> String {
        self.windows
            .get(&window)
            .map(|window| window.title.clone())
            .unwrap_or_default()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenWindow => {
                let (_id, open) = window::open(window::Settings {
                    icon: Some(load_icon()),
                    ..Default::default()
                });
                open.map(Message::WindowOpened)
            }
            Message::WindowOpened(id) => {
                let window = Window::new("new".to_string(), View::Main);
                self.windows.insert(id, window);
                Task::none()
            }
            Message::WindowClosed(id) => {
                self.windows.remove(&id);

                if self.windows.is_empty() {
                    iced::exit()
                } else {
                    Task::none()
                }
            }
            Message::AboutPressed(id) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.previous_view = Some(window.view);
                    window.view = View::About;
                }
                Task::none()
            }
            Message::About(about::AboutMessage::Back) => {
                if let Some(window) = self.windows.values_mut().find(|w| w.view == View::About) {
                    if let Some(previous_view) = window.previous_view {
                        window.view = previous_view;
                        window.previous_view = None;
                    }
                }
                Task::none()
            }
            Message::About(message) => about::About::update(message).map(Message::About),
            Message::ChatPressed(id) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.previous_view = Some(window.view);
                    window.view = View::Chat;
                }
                Task::none()
            }

            Message::Chat(chat::ChatMessage::Back) => {
                if let Some(window) = self.windows.values_mut().find(|w| w.view == View::Chat) {
                    if let Some(previous_view) = window.previous_view {
                        window.view = previous_view;
                        window.previous_view = None;
                    }
                }
                Task::none()
            }

            Message::Chat(message) => chat::Chat::update(message).map(Message::Chat),

            Message::OpenSettingsWindow => {
                let settings = window::Settings {
                    size: iced::Size::new(400.0, 300.0),
                    ..window::Settings::default()
                };
                let (_id, open) = window::open(settings);
                open.map(Message::SettingsWindowOpened)
            }
            Message::SettingsWindowOpened(id) => {
                let window = Window::new("Settings".to_string(), View::Settings);
                self.windows.insert(id, window);
                Task::none()
            }
            Message::Settings(settings::SettingsMessage::CloseSettingsWindow(id)) => {
                println!("CloseSettingsWindow");
                self.windows.remove(&id);
                window::close(id)
            }
            Message::Settings(message) => {
                settings::Settings::update(message).map(Message::Settings)
            }
            Message::TwitchIdChanged(new_id) => {
                self.global_twitch_id = new_id;
                println!("twitch_id");
                Task::none()
            }
            Message::YoutubeIdChanged(new_id) => {
                self.global_youtube_id = new_id;
                println!("youtube_id");
                Task::none()
            }
        }
    }

    fn view(&self, window_id: window::Id) -> Element<Message> {
        let window = self.windows.get(&window_id);

        let is_main_window = self
            .windows
            .keys()
            .next()
            .map(|id| *id == window_id)
            .unwrap_or(false);

        let is_main_view = window.map(|w| w.view == View::Main).unwrap_or(false);

        let window_content = window
            .map(|w| w.view(window_id))
            .unwrap_or_else(|| horizontal_space().into());

        if is_main_window && is_main_view {
            let twitch_input =
                text_input("Twitch ID", &self.global_twitch_id).on_input(Message::TwitchIdChanged);
            let youtube_input = text_input("YouTube ID", &self.global_youtube_id)
                .on_input(Message::YoutubeIdChanged);

            let form = column![text("🔧 Configuración global"), twitch_input, youtube_input]
                .spacing(16)
                .padding(20);

            return center(column![form, window_content].spacing(40)).into();
        }

        center(window_content).into()
    }

    fn theme(&self, window: window::Id) -> Theme {
        if let Some(window) = self.windows.get(&window) {
            window.theme.clone()
        } else {
            Theme::default()
        }
    }

    fn scale_factor(&self, window: window::Id) -> f64 {
        self.windows
            .get(&window)
            .map(|window| window.current_scale)
            .unwrap_or(1.0)
    }

    fn subscription(&self) -> Subscription<Message> {
        window::close_events().map(Message::WindowClosed)
    }
}

impl Window {
    fn new(title: String, view: View) -> Self {
        Self {
            title,
            current_scale: 1.0,
            theme: Theme::ALL[1 % Theme::ALL.len()].clone(),
            view,
            previous_view: None,
        }
    }

    fn view(&self, window_id: window::Id) -> Element<Message> {
        match self.view {
            View::Main => {
                let layout = column![
                    button("About").on_press(Message::AboutPressed(window_id)),
                    button("Settings").on_press(Message::OpenSettingsWindow),
                    button("Continue...").on_press(Message::ChatPressed(window_id))
                ]
                .spacing(20)
                .align_x(Center);

                scrollable(layout).into()
            }
            View::About => about::About::view().map(Message::About),
            View::Settings => settings::Settings::view(window_id),
            View::Chat => chat::Chat::view().map(Message::Chat),
        }
    }
}
