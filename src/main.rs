use iced::widget::{button, center, column, horizontal_space, scrollable, text};
use iced::{window, Task};
use iced::{Center, Fill, Subscription, Theme};
use utils::load_icon;

use std::collections::BTreeMap;
type Renderer = iced::Renderer;
type Element<'a, Message> = iced::Element<'a, Message, Theme, Renderer>;
mod about;
mod settings;
mod utils;

fn main() -> iced::Result {
    iced::daemon(Example::title, Example::update, Example::view)
        .subscription(Example::subscription)
        .theme(Example::theme)
        .scale_factor(Example::scale_factor)
        .run_with(Example::new)
}

struct Example {
    windows: BTreeMap<window::Id, Window>,
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

    OpenSettingsWindow,
    SettingsWindowOpened(window::Id),
    Settings(settings::SettingsMessage),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum View {
    Main,
    About,
    Settings,
}

impl Example {
    fn new() -> (Self, Task<Message>) {
        let (_id, open) = window::open(window::Settings {
            icon: Some(load_icon()),
            transparent: true,
            ..Default::default()
        });

        (
            Self {
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
        }
    }

    fn view(&self, window_id: window::Id) -> Element<Message> {
        if let Some(window) = self.windows.get(&window_id) {
            center(window.view(window_id)).into()
        } else {
            horizontal_space().into()
        }
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
                let new_window_button = button(text("New Window")).on_press(Message::OpenWindow);
                let about_button = button(text("About")).on_press(Message::AboutPressed(window_id));
                let settings_button =
                    button(text("Settings")).on_press(Message::OpenSettingsWindow);

                let content = scrollable(
                    column![new_window_button, about_button, settings_button]
                        .spacing(50)
                        .width(Fill)
                        .align_x(Center),
                );

                content.into()
            }
            View::About => about::About::view().map(Message::About),
            View::Settings => settings::Settings::view(window_id),
        }
    }
}
