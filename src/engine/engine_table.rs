use iced::{
    Element, Length, Renderer, Size, Theme,
    widget::{button, container, horizontal_space, scrollable, text, text_input},
};
use iced_table::{Table, table};

use crate::{
    engine::{engine_args::EngineArgs, engine_paths::EnginePaths},
    message::Message,
};

pub struct EngineTable {
    pub columns: Vec<Column>,
    pub rows: Vec<EngineArgs>,
    pub header: scrollable::Id,
    pub body: scrollable::Id,
    pub footer: scrollable::Id,
}

impl Default for EngineTable {
    fn default() -> Self {
        EngineTable {
            columns: vec![
                Column::new(ColumnKind::Index),
                Column::new(ColumnKind::EngineName),
                Column::new(ColumnKind::Args),
                Column::new(ColumnKind::Path),
                Column::new(ColumnKind::Delete),
            ],
            rows: vec![],
            header: scrollable::Id::unique(),
            body: scrollable::Id::unique(),
            footer: scrollable::Id::unique(),
        }
    }
}

impl EngineTable {
    pub fn change_data(&mut self, new_data: Vec<EngineArgs>) {
        self.rows = new_data;
    }
}

#[derive(Clone)]
pub struct Column {
    kind: ColumnKind,
    width: f32,
    resize_offset: Option<f32>,
}
#[derive(Clone)]
pub enum ColumnKind {
    Index,
    EngineName,
    Args,
    Path,
    Delete,
}

impl Column {
    fn new(kind: ColumnKind) -> Self {
        let width = match kind {
            ColumnKind::Index => 60.0,
            ColumnKind::EngineName => 150.0,
            ColumnKind::Args => 250.0,
            ColumnKind::Path => 350.0,
            ColumnKind::Delete => 100.0,
        };

        Self {
            kind,
            width,
            resize_offset: None,
        }
    }
}

impl<'a> table::Column<'a, Message, Theme, Renderer> for Column {
    type Row = EngineArgs;

    fn header(&'a self, _col_index: usize) -> Element<'a, Message> {
        let content = match self.kind {
            ColumnKind::Index => "Index",
            ColumnKind::EngineName => "Name",
            ColumnKind::Args => "Args",
            ColumnKind::Path => "Path",
            ColumnKind::Delete => "Delete",
        };

        container(text(content)).center_y(24).into()
    }

    fn cell(
        &'a self,
        _col_index: usize,
        row_index: usize,
        row: &'a EngineArgs,
    ) -> Element<'a, Message> {
        let content: Element<_> = match self.kind {
            ColumnKind::Index => text(row_index).into(),
            ColumnKind::EngineName => text_input("", row.name.as_str())
                .on_input(move |name| Message::ChangeEngineName(row_index, name))
                .into(),
            ColumnKind::Args => text_input("", &row.args.as_str())
                .on_input(move |args| Message::ChangeEngineArgs(row_index, args))
                .into(),
            ColumnKind::Path => text(row.path.as_str()).into(),
            ColumnKind::Delete => button(text("Delete"))
                .on_press(Message::DeleteEngine(row_index))
                .into(),
        };

        container(content).width(Length::Fill).center_y(32).into()
    }

    fn footer(&'a self, _col_index: usize, rows: &'a [EngineArgs]) -> Option<Element<'a, Message>> {
        let content = match self.kind {
            ColumnKind::EngineName => {
                let btn = button(text("添加引擎")).on_press(Message::AddEngineButton);
                Element::from(btn)
            }
            ColumnKind::Args => {
                let btn = button(text("关闭")).on_press(Message::CloseEngineManager);
                Element::from(btn)
            }
            _ => horizontal_space().into(),
        };

        Some(container(content).center_y(30).into())
    }

    fn width(&self) -> f32 {
        self.width
    }

    fn resize_offset(&self) -> Option<f32> {
        self.resize_offset
    }
}
