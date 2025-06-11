use iced::Color;

use crate::chessboard::chessboard_trait::ChessboardTrait;

pub struct Gomoku {}

impl Gomoku {
    pub fn new() -> Self {
        Gomoku {}
    }
}

impl ChessboardTrait for Gomoku {
    fn get_length(&self) -> (u32, u32) {
        (15, 15)
    }
    fn get_pieces(&self) -> Vec<Vec<(Color, Color)>> {
        vec![vec![(Color::WHITE, Color::BLACK); 15]; 15]
    }
}
