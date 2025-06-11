#[derive(Debug, Clone)]
pub enum Message {
    GoBoard(i32, i32),
    NewBoard,
}
