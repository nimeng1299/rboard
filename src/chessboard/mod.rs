pub mod chessboard_trait;
pub mod gomoku;
pub mod zhenqi;

use crate::chessboard::chessboard_trait::ChessboardTrait;

pub fn get_chessboard(name: String) -> Box<dyn ChessboardTrait> {
    match name.as_str() {
        "gomoku" => Box::new(gomoku::Gomoku::new()),
        "zhenqi" => Box::new(zhenqi::Zhenqi::new()),
        _ => Box::new(gomoku::Gomoku::new()),
    }
}

//(名字, 唯一名字)
pub fn get_all_board_names() -> Vec<(String, String)> {
    vec![
        ("Gomoku 15 * 15".to_string(), "gomoku".to_string()),
        ("Zhenqi 8 * 8".to_string(), "zhenqi".to_string()),
    ]
}

//棋子转坐标 例如:B8 -> 1 , (y - 8)
pub fn get_piece(size: &String, _x: u32, y: u32, ingore_i: bool) -> Option<(u32, u32)> {
    if *size == "pass".to_string() {
        return None;
    }
    if size.len() < 2 {
        return None;
    }
    let mut chars = size.chars();

    let mut x1 = 9999;
    let mut y1 = 9999;
    if let Some(c) = chars.next() {
        x1 = c as u32 - 'A' as u32;
        if ingore_i && x1 >= 'I' as u32 - 'A' as u32 {
            x1 = x1 - 1;
        }
    }
    while let Some(c) = chars.next() {
        if let Some(num) = c.to_digit(10) {
            if y1 == 9999 {
                y1 = num;
            } else {
                y1 = y1 * 10 + num;
            }
        } else {
            break;
        }
    }
    if let Some(y) = y.checked_sub(y1) {
        Some((x1, y))
    } else {
        None
    }
}
