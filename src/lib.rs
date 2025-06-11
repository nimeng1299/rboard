pub mod board;
pub mod chessboard;
pub mod engine;
pub mod message;
pub mod style;

use iced::widget::{canvas, column, progress_bar, text};
use iced::{Background, Border, Color, Font, Length, Task};
use iced_aw::menu::{Item, Menu};
use iced_aw::{menu_bar, menu_items};
use rfd::AsyncFileDialog;

use crate::board::{Board, BoardState};
use crate::engine::engine_paths::EnginePaths;
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
    engine_path: EnginePaths,
}

impl RBoard {
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::GoBoard(x, y) => {
                self.board_state.chessboard.go(x, y);
            }
            Message::NewBoard => {
                self.board_state.chessboard.new_board();
            }
            Message::AddEngineButton => {
                return Task::perform(
                    AsyncFileDialog::new()
                        .add_filter("engine", &["exe"])
                        .set_title("添加引擎...")
                        .pick_file(),
                    |result| Message::AddEngige(result),
                );
            }
            Message::AddEngige(result) => match result {
                Some(path) => {
                    println!("{:?}", path);
                }
                None => {
                    eprintln!("Error adding engine path");
                }
            },
        }
        iced::Task::none()
    }

    fn view(&self) -> iced::Element<Message> {
        let menu_template = |items| Menu::new(items).max_width(100.0).offset(15.0).spacing(3.0);
        #[rustfmt::skip]
        let menu_bar = menu_bar!((
            text("菜单"),
            menu_template(menu_items!(
                (styles::button::secondary_menu_button("新棋盘", Message::NewBoard))
                (styles::button::secondary_menu_button("添加引擎...", Message::AddEngineButton))
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
