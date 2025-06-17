use iced::{
    Element, Length, Renderer, Size, Theme,
    widget::{button, container, horizontal_space, scrollable, text, text_input},
};
use iced_table::{Table, table};

use crate::{engine::analyze::Analyze, message::Message};

pub struct AnalyzesTable {
    pub columns: Vec<Column>,
    pub rows: Vec<Analyze>,
    pub header: scrollable::Id,
    pub body: scrollable::Id,
    pub footer: scrollable::Id,
}

impl Default for AnalyzesTable {
    fn default() -> Self {
        AnalyzesTable {
            columns: vec![
                Column::new(ColumnKind::Order),
                Column::new(ColumnKind::Move),
                Column::new(ColumnKind::Visits),
                Column::new(ColumnKind::Winrate),
                Column::new(ColumnKind::Pv),
                Column::new(ColumnKind::PvVisits),
            ],
            rows: vec![],
            header: scrollable::Id::unique(),
            body: scrollable::Id::unique(),
            footer: scrollable::Id::unique(),
        }
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
    Order,
    Move,
    Visits,
    Winrate,
    Pv,
    PvVisits,
}

impl Column {
    fn new(kind: ColumnKind) -> Self {
        let width = match kind {
            ColumnKind::Order => 80.0,
            ColumnKind::Move => 80.0,
            ColumnKind::Visits => 80.0,
            ColumnKind::Winrate => 80.0,
            ColumnKind::Pv => 400.0,
            ColumnKind::PvVisits => 80.0,
        };

        Self {
            kind,
            width,
            resize_offset: None,
        }
    }
}

impl<'a> table::Column<'a, Message, Theme, Renderer> for Column {
    type Row = Analyze;

    fn header(&'a self, _col_index: usize) -> Element<'a, Message> {
        let content = match self.kind {
            ColumnKind::Order => "order",
            ColumnKind::Move => "move",
            ColumnKind::Visits => "visits",
            ColumnKind::Winrate => "winrate",
            ColumnKind::Pv => "pv",
            ColumnKind::PvVisits => "pv visits",
        };

        container(text(content)).center_y(24).into()
    }

    fn cell(
        &'a self,
        _col_index: usize,
        row_index: usize,
        row: &'a Analyze,
    ) -> Element<'a, Message> {
        let content: Element<_> = match self.kind {
            ColumnKind::Order => text(row_index).into(),
            ColumnKind::Move => text(row.move_.to_string()).into(),
            ColumnKind::Visits => text(row.visits).into(),
            ColumnKind::Winrate => text(row.winrate).into(),
            ColumnKind::Pv => text(row.pv.join(" ")).into(),
            ColumnKind::PvVisits => text(row.pv_visits).into(),
        };

        container(content).width(Length::Fill).center_y(32).into()
    }

    fn footer(&'a self, _col_index: usize, _rows: &'a [Analyze]) -> Option<Element<'a, Message>> {
        None
    }

    fn width(&self) -> f32 {
        self.width
    }

    fn resize_offset(&self) -> Option<f32> {
        self.resize_offset
    }
}
