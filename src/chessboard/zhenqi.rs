use iced::Color;

use crate::chessboard::chessboard_trait::ChessboardTrait;

pub struct Zhenqi {
    board: Vec<Vec<i32>>,
    current_player: i32,
    size_x: u32,
    size_y: u32,
}
impl Zhenqi {
    pub fn new() -> Self {
        Zhenqi {
            board: vec![vec![-1; 8]; 8],
            current_player: 0,
            size_x: 8,
            size_y: 8,
        }
    }
    fn get_state(&self, x: i32, y: i32) -> PieceState {
        if x < 0 || x > self.size_x as i32 - 1 || y < 0 || y > self.size_y as i32 - 1 {
            return PieceState::Outside;
        }
        match self.board[x as usize][y as usize] {
            1 => PieceState::Piece,
            0 => PieceState::Piece,
            -1 => PieceState::Empty,
            _ => PieceState::Outside,
        }
    }
}

impl ChessboardTrait for Zhenqi {
    fn get_length(&self) -> (u32, u32) {
        (self.size_x, self.size_y)
    }
    fn get_pieces(&self) -> Vec<Vec<Option<(iced::Color, iced::Color)>>> {
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
        if x < 0 || x > self.size_x as i32 - 1 || y < 0 || y > self.size_y as i32 - 1 {
            return None;
        }
        if self.board[x as usize][y as usize] != -1 {
            return None;
        }
        self.board[x as usize][y as usize] = self.current_player;
        let direction = vec![(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1)];
        for (dx, dy) in direction {
            let nx = x + dx;
            let ny = y + dy;
            if self.get_state(nx, ny) == PieceState::Piece {
                let nnx = nx + dx;
                let nny = ny + dy;
                if self.get_state(nnx, nny) == PieceState::Empty {
                    println!("go 1");
                    self.board[nnx as usize][nny as usize] = self.board[nx as usize][nx as usize];
                    self.board[nx as usize][nx as usize] = -1;
                }
                if self.get_state(nnx, nny) == PieceState::Outside {
                    println!("go 2");
                    self.board[nx as usize][nx as usize] = -1;
                }
            }
        }
        let p = if self.current_player == 0 { "B" } else { "W" };
        self.current_player = 1 - self.current_player;
        let mut p_x = x;
        if p_x >= 'I' as i32 - 'A' as i32 {
            p_x = p_x + 1;
        }
        Some(format!(
            "play {} {}{}",
            p,
            ('A' as i32 + p_x) as u8 as char,
            self.size_y as i32 - y
        ))
    }
    fn new_board(&mut self) {
        self.board = vec![vec![-1; 15]; 15];
        self.current_player = 0;
    }
}

#[derive(PartialEq)]
enum PieceState {
    Piece,
    Empty,
    Outside,
}
