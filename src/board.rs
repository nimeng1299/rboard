use iced::{
    Color, Renderer, Theme,
    mouse::Cursor,
    widget::canvas::{self, Geometry},
};

use crate::{
    chessboard::{chessboardTrait::ChessboardTrait, gomoku::Gomoku},
    message::Message,
};

pub struct Board {}

impl<Message> canvas::Program<Message> for Board {
    type State = BoardState;

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: iced::Rectangle,
        cursor: Cursor,
    ) -> Vec<Geometry<Renderer>> {
        // let p = cursor.position_in(bounds);
        // println!("{:#?}", p);

        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let circle = canvas::Path::circle(frame.center(), 10.0);
        frame.fill(&circle, Color::BLACK);
        vec![frame.into_geometry()]
    }
}

pub struct BoardState {
    chessboard: Box<dyn ChessboardTrait>,
}

impl Default for BoardState {
    fn default() -> Self {
        BoardState {
            chessboard: Box::new(Gomoku::new()),
        }
    }
}
