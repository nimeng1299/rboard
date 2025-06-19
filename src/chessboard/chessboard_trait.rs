use iced::Color;

/// 棋盘操作的通用接口，定义了获取棋盘信息和下棋行为的方法。
/// api不稳定，也许会改动
pub trait ChessboardTrait {
    /// 获取棋盘的尺寸（宽度，高度）。
    ///
    /// # 返回值
    /// 返回一个包含棋盘宽度和高度的元组 `(u32, u32)`。
    fn get_length(&self) -> (u32, u32);

    /// 获取当前棋盘上的棋子状态。
    ///
    /// # 返回值
    /// 返回一个二维向量表示棋盘每个位置的棋子状态：
    /// - `None` 表示该位置为空。
    /// - `Some((inner_color, outer_color))` 表示该位置有一个棋子，其内圆和外框颜色分别为 `inner_color` 和 `outer_color`。
    /// - `None`：落子失败（如非法位置等）。
    fn get_pieces(&self) -> Vec<Vec<Option<(Color, Color)>>>;

    /// 尝试在指定坐标 `(x, y)` 下棋。
    ///
    /// # 参数
    /// - `x`: 横坐标
    /// - `y`: 纵坐标
    ///
    /// # 返回值
    /// - `Some(String)`：下棋成功，返回格式化的落子信息，例如 `"play W G3"`。用于引擎走子
    /// - `None`：落子失败（如非法位置等）。
    fn go(&mut self, x: i32, y: i32) -> Option<String>;

    /// 重置棋盘，开始新的一局。
    fn new_board(&mut self);

    /// 获取当前执棋的玩家。
    ///
    /// # 返回值
    /// 返回当前轮到的玩家（例如黑方或白方）。
    fn get_player(&self) -> Player;
}

#[derive(PartialEq)]
pub enum Player {
    Black,
    White,
}
