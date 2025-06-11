use iced::Color;

use crate::chessboard::chessboard_trait::ChessboardTrait;

pub struct Gomoku {
    // -1: empty 0: black 1: white
    board: Vec<Vec<i32>>,
    current_player: i32,
}

impl Gomoku {
    pub fn new() -> Self {
        Gomoku {
            board: vec![vec![-1; 15]; 15],
            current_player: 0,
        }
    }
}

impl ChessboardTrait for Gomoku {
    fn get_length(&self) -> (u32, u32) {
        (15, 15)
    }
    fn get_pieces(&self) -> Vec<Vec<Option<(Color, Color)>>> {
        let black = (Color::BLACK, Color::WHITE);
        let white = (Color::WHITE, Color::BLACK);
        self.board
            .clone()
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|cell| match cell {
                        0 => Some(black),
                        1 => Some(white),
                        _ => None,
                    })
                    .collect()
            })
            .collect()
    }
    fn go(&mut self, x: i32, y: i32) -> Option<String> {
        if x < 0 || x > 14 || y < 0 || y > 14 {
            return None;
        }
        if self.board[x as usize][y as usize] != -1 {
            return None;
        }
        self.board[x as usize][y as usize] = self.current_player;
        self.current_player = 1 - self.current_player;
        let p = if self.current_player == 0 { "B" } else { "W" };
        Some(format!(
            "paly {} {}{})",
            p,
            ('A' as i32 + x) as u8 as char,
            15 - y
        ))
    }
}
