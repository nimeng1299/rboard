use iced::widget::{scrollable, text_editor};
use rfd::FileHandle;

#[derive(Debug, Clone)]
pub enum Message {
    NewBoard,
    GoBoard(i32, i32),
    ChangeBoard(String),
    AddEngineButton,
    AddEngige(Option<FileHandle>),
    ChangeEngine(usize),
    ChangeEngineSettingSelectionList(usize, String),
    OpenEngineManager,
    CloseEngineManager,
    EngineTableSyncHeader(scrollable::AbsoluteOffset),
    ChangeEngineName(usize, String),
    ChangeEngineArgs(usize, text_editor::Action),
    DeleteEngine,
    EngineOutputSelected(usize, String),
    EngineSender(iced::futures::channel::mpsc::Sender<String>),
    EngineReceiveOutput(String),
}
