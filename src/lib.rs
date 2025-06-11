pub mod board;
pub mod chessboard;
pub mod message;
pub mod style;

use iced::widget::toggler::default;
use iced::widget::{button, canvas, column, progress_bar, text};
use iced::{Background, Border, Color, Font, Length};
use iced_aw::menu::{Item, Menu};
use iced_aw::{menu_bar, menu_items};

use crate::board::{Board, BoardState};
use crate::message::Message;

use crate::style as styles;

pub fn start() -> iced::Result {
    iced::application("rboard", RBoard::update, RBoard::view)
        .font(include_bytes!("E:\\85W.ttf"))
        .default_font(Font::with_name("汉仪文黑"))
        .run()
}

#[derive(Default)]
struct RBoard {
    board_state: BoardState,
}

impl RBoard {
    fn update(&mut self, message: Message) {
        match message {
            Message::GoBoard(x, y) => {
                self.board_state.chessboard.go(x, y);
            }
            Message::NewBoard => {}
            _ => {}
        }
    }

    fn view(&self) -> iced::Element<Message> {
        let menu_template = |items| Menu::new(items).max_width(100.0).offset(15.0).spacing(3.0);
        let menu_bar = menu_bar!((
            text("菜单"),
            menu_template(menu_items!(
                (styles::button::secondary_menu_button("新棋盘", Message::NewBoard))
            ))
        ));

        let board = canvas(Board {
            count: self.board_state.chessboard.get_length(),
            pieces: self.board_state.chessboard.get_pieces(),
        })
        .width(Length::Fill)
        .height(Length::Fill);
        let rate = progress_bar(0.0..=100.0, 35.0).style(|_| progress_bar::Style {
            background: Background::Color(Color::WHITE),
            bar: Background::Color(Color::BLACK),
            border: Border::default()
                .color(Color::from_rgb8(211, 211, 211))
                .width(2.0),
        });
        // Render the chessboard and pieces
        column![menu_bar, board, rate].padding(10).spacing(5).into()
    }
}
