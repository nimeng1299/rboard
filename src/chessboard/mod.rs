pub mod chessboard_trait;
pub mod gomoku;
pub mod zhenqi;

use crate::chessboard::chessboard_trait::ChessboardTrait;

/// 根据给定名称创建并返回对应的棋盘实例。
///
/// # 参数
/// - `name`: 棋盘类型的唯一标识字符串，例如 `"gomoku"`等。
///   - 如果传入的 `name` 不在支持的列表中，当前实现会默认返回 `"gomoku"` 类型的棋盘实例。
///
/// # 返回值
/// 返回一个 `Box<dyn ChessboardTrait>`，即指向实现了 `ChessboardTrait` 的具体棋盘对象的堆分配指针。
///
/// # 示例
/// ```
/// let board = get_chessboard("gomoku".to_string());
/// // board: Box<dyn ChessboardTrait>
/// ```
pub fn get_chessboard(name: String) -> Box<dyn ChessboardTrait> {
    match name.as_str() {
        "gomoku" => Box::new(gomoku::Gomoku::new()),
        "zhenqi" => Box::new(zhenqi::Zhenqi::new()),
        _ => Box::new(gomoku::Gomoku::new()),
    }
}

/// 获取所有可用棋盘类型的显示名称和唯一标识名称列表。
///
/// # 返回值
/// 返回一个 `Vec<(String, String)>`，其中每个元素是一个二元组：
/// - 第一个 `String`：供界面或日志展示的“友好名称”，例如 `"Gomoku 15 * 15"`
/// - 第二个 `String`：对应的唯一标识名称，用于传入 `get_chessboard` 等函数，例如 `"gomoku"`
///
/// # 示例
/// ```
/// let names = get_all_board_names();
/// for (display, key) in names {
///     println!("显示名称: {}, 唯一标识: {}", display, key);
/// }
/// ```
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
