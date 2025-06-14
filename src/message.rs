use iced::widget::scrollable;
use rfd::FileHandle;

#[derive(Debug, Clone)]
pub enum Message {
    NewBoard,
    GoBoard(i32, i32),
    AddEngineButton,
    AddEngige(Option<FileHandle>),
    ChangeEngine(String),
    OpenEngineManager,
    CloseEngineManager,
    EngineTableSyncHeader(scrollable::AbsoluteOffset),
    ChangeEngineName(usize, String),
    ChangeEngineArgs(usize, String),
    DeleteEngine(usize),
}
