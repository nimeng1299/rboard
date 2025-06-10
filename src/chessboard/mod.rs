pub mod chessboard_trait;
pub mod gomoku;

use crate::chessboard::chessboard_trait::ChessboardTrait;

pub fn get_chessboard(name: String) -> Box<dyn ChessboardTrait> {
    match name.as_str() {
        _ => Box::new(gomoku::Gomoku::new()),
    }
}
