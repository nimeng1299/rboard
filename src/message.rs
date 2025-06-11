#[derive(Debug, Clone)]
pub enum Message {
    Engine(String),
    NewBoard,
}
