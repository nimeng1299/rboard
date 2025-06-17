use iced::widget::scrollable;
use rfd::FileHandle;

#[derive(Debug, Clone)]
pub enum Message {
    NewBoard,
    GoBoard(i32, i32),
    AddEngineButton,
    AddEngige(Option<FileHandle>),
    ChangeEngine(usize),
    OpenEngineManager,
    CloseEngineManager,
    EngineTableSyncHeader(scrollable::AbsoluteOffset),
    ChangeEngineName(usize, String),
    ChangeEngineArgs(usize, String),
    DeleteEngine(usize),
    EngineOutputSelected(usize, String),
    EngineSender(iced::futures::channel::mpsc::Sender<String>),
    EngineReceiveOutput(String),
}
