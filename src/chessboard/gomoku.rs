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
}
