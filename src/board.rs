use iced::{
    Renderer, Theme,
    mouse::Cursor,
    widget::canvas::{self, Geometry},
};

use crate::message::Message;

pub struct Board {}

impl canvas::Program<Message> for Board {
    type State = BoardState;

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: iced::Rectangle,
        cursor: Cursor,
    ) -> Vec<Geometry<Renderer>> {
        vec![]
    }
}

#[derive(Default)]
pub struct BoardState {
    // Define the state of the chessboard, pieces, etc.
}
