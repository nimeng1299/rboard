pub mod board;
pub mod chessboard;
pub mod message;

use iced::Length;
use iced::widget::{canvas, column, text};

use crate::board::Board;
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
        let board = canvas(Board {}).width(Length::Fill).height(Length::Fill);
        // Render the chessboard and pieces
        column![text("board"), board].padding(10).spacing(5).into()
    }
}
