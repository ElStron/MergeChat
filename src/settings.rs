use crate::Element;
use crate::Message;
use iced::widget::text;
use iced::{window, Task};

#[derive(Default)]
pub struct Settings;

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    Close,
    CloseSettingsWindow(window::Id),
}

impl Settings {
    pub fn new() -> Settings {
        Settings
    }

    pub fn view<'a>(window_id: window::Id) -> Element<'a, Message> {
        // Cambia el tipo de retorno a Element<'a, Message>
        iced::widget::column![
            text("Settings dialog").size(30),
            iced::widget::button("Close").on_press(Message::Settings(SettingsMessage::Close)), // Envía Message::Settings
            iced::widget::button("Close Settings").on_press(Message::Settings(
                SettingsMessage::CloseSettingsWindow(window_id)
            ))  // Envía Message::Settings
        ]
        .into()
    }

    pub fn update(message: SettingsMessage) -> Task<SettingsMessage> {
        match message {
            SettingsMessage::Close => {
                println!("SettingsMessage::Close");
                Task::none()
            }
            SettingsMessage::CloseSettingsWindow(_id) => Task::none(),
        }
    }
}

