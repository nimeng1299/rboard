pub mod board;
pub mod message;

use iced::widget::text;

use crate::message::Message;

pub fn start() -> iced::Result {
    iced::run("rboard", RBoard::update, RBoard::view)
}

#[derive(Default)]
struct RBoard {
    // Define the state of the chessboard, pieces, etc.
}

impl RBoard {
    fn update(&mut self, message: Message) {
        // Update the state based on user input or game logic
    }

    fn view(&self) -> iced::Element<Message> {
        // Render the chessboard and pieces
        text("rboard").into()
    }
}
