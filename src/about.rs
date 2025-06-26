use iced::widget::text;
use iced::Task;

use crate::Element;
#[derive(Debug, Clone)]
pub enum AboutMessage {
    Test,
    Back,
}

#[derive(Default)]
pub struct About;

impl About {
    pub fn view<'a>() -> Element<'a, AboutMessage> {
        iced::widget::column![
            text("About dialog").size(30),
            iced::widget::button("Close").on_press(AboutMessage::Test),
            iced::widget::button("Back").on_press(AboutMessage::Back)
        ]
        .into()
    }

    pub fn update(message: AboutMessage) -> Task<AboutMessage> {
        match message {
            AboutMessage::Test => {
                println!("AboutMessage::Test");
                Task::none()
            }
            AboutMessage::Back => Task::none(),
        }
    }
}
