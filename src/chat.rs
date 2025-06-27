use iced::widget::text;
use iced::Task;

//use crate::about::AboutMessage;
use crate::Element;
#[derive(Debug, Clone)]
pub enum ChatMessage {
    Test,
    Back,
}

#[derive(Default)]
pub struct Chat;

impl Chat {
    pub fn view<'a>() -> Element<'a, ChatMessage> {
        iced::widget::column![
            text("Chat dialog").size(30),
            iced::widget::button("Close").on_press(ChatMessage::Test),
            iced::widget::button("Back").on_press(ChatMessage::Back)
        ]
        .into()
    }

    pub fn update(message: ChatMessage) -> Task<ChatMessage> {
        match message {
            ChatMessage::Test => {
                println!("AboutMessage::Test");
                Task::none()
            }
            ChatMessage::Back => Task::none(),
        }
    }
}
