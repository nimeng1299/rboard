pub mod board;
pub mod chessboard;
pub mod engine;
pub mod message;
pub mod style;

use std::sync::{Arc, Mutex};
use std::thread::{self, Thread};
use std::time::Duration;

use iced::futures::{self, Stream, stream};
use iced::widget::{Column, Row, canvas, column, progress_bar, responsive, row, text};
use iced::{Background, Border, Color, Font, Length, Subscription, Task, time};
use iced_aw::menu::{Item, Menu};
use iced_aw::{menu_bar, menu_items, selection_list};
use iced_table::table;
use rfd::AsyncFileDialog;

use crate::board::{Board, BoardState};
use crate::engine::engine_paths::EnginePaths;
use crate::engine::engine_table::{self, EngineTable};
use crate::engine::gtp::GTP;
use crate::message::Message;

use crate::style as styles;

pub fn start() -> iced::Result {
    iced::application(RBoard::title, RBoard::update, RBoard::view)
        .font(include_bytes!("E:\\85W.ttf"))
        .default_font(Font::with_name("汉仪文黑"))
        .run()
}

struct RBoard {
    board_state: BoardState,
    engine_path: EnginePaths,

    show_engine_manager: bool,
    engine_table_info: EngineTable,

    engine: Option<GTP>,
    engine_msg: Vec<String>,
    engine_analyze: String,

    engine_tx: Arc<std::sync::mpsc::Sender<String>>,
    engine_rx: Arc<std::sync::mpsc::Receiver<String>>,
}

impl Default for RBoard {
    fn default() -> Self {
        let (engine_tx, engine_rx) = std::sync::mpsc::channel::<String>();
        // let engine_output_handle = thread::spawn(move || {
        //     loop {
        //         if let Ok(mut m_gtp) = engine_clone.try_lock() {
        //             if let Some(gtp) = m_gtp.take() {
        //                 let data_clone = Arc::clone(&gtp.data);
        //                 if let Ok(mut lock) = data_clone.try_lock() {
        //                     while let Some(item) = lock.pop_front() {
        //                         if !item.starts_with("info") {
        //                             engine_msg_clone.lock().unwrap().push(item);
        //                         } else {
        //                             *engine_analyze_clone.lock().unwrap() = item;
        //                         }
        //                     }
        //                 }
        //                 *m_gtp = Some(gtp);
        //             }
        //         }
        //         thread::sleep(Duration::from_millis(10));
        //     }
        // });
        Self {
            board_state: Default::default(),
            engine_path: Default::default(),
            show_engine_manager: false,
            engine_table_info: Default::default(),
            engine: None,
            engine_msg: Vec::new(),
            engine_analyze: String::new(),
            engine_tx: Arc::new(engine_tx),
            engine_rx: Arc::new(engine_rx),
        }
    }
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
                    let path = path.path().to_path_buf();
                    let _ = self.engine_path.add(path);
                    self.engine_table_info
                        .change_data(self.engine_path.get_all_paths());
                }
                None => {
                    eprintln!("Error adding engine path");
                }
            },
            Message::OpenEngineManager => {
                self.engine_table_info
                    .change_data(self.engine_path.get_all_paths());
                self.show_engine_manager = true;
            }
            Message::CloseEngineManager => self.show_engine_manager = false,
            Message::ChangeEngineName(index, name) => {
                let _ = self.engine_path.change_name(index, name);
                self.engine_table_info
                    .change_data(self.engine_path.get_all_paths());
            }
            Message::ChangeEngineArgs(index, args) => {
                let _ = self.engine_path.change_args(index, args);
                self.engine_table_info
                    .change_data(self.engine_path.get_all_paths());
            }
            Message::DeleteEngine(index) => {
                let _ = self.engine_path.delete(index);
                self.engine_table_info
                    .change_data(self.engine_path.get_all_paths());
            }
            Message::ChangeEngine(index) => {
                let args = self.engine_path.get_all_paths()[index].clone();
                let gtp = GTP::start(
                    &args.path.as_str(),
                    &args.args.as_str(),
                    Arc::clone(&self.engine_tx),
                );
                match gtp {
                    Ok(gtp) => {
                        let _ = gtp.send_command("name".to_string());
                        let _ = gtp.send_command("version".to_string());
                        let _ = gtp.send_command("list_commands".to_string());
                        let (x, _) = self.board_state.chessboard.get_length();
                        let _ = gtp.send_command(format!("boardsize {}", x));
                        let _ = gtp.send_command("kata-get-rules".to_string());
                        let _ = gtp.send_kata_analyze();
                        self.engine = Some(gtp);
                        self.engine_path.current_path = Some(index as i32);
                    }
                    Err(e) => {
                        println!("gtp load err: {}", e);
                        self.engine_path.current_path = None;
                    }
                }
                self.engine_msg = vec![];
            }
            _ => {}
        }

        iced::Task::none()
    }

    fn view(&self) -> iced::Element<Message> {
        let menu_template = |items| Menu::new(items).max_width(200.0).offset(15.0).spacing(3.0);
        let mut engine_path = vec![];
        let e_p = self.engine_path.get_all_paths();
        let mut e_len = 15;
        for i in 0..e_p.len() {
            let s = e_p[i].path.clone();
            e_len = e_len.max(s.len());
            engine_path.push(Item::new(styles::button::secondary_menu_button(
                text(s.clone()),
                Message::ChangeEngine(i),
            )));
        }
        engine_path.push(Item::new(styles::button::secondary_menu_button(
            "引擎管理",
            Message::OpenEngineManager,
        )));
        #[rustfmt::skip]
        let menu_bar = menu_bar!(
            (
            text("菜单"),
            menu_template(menu_items!(
                (styles::button::secondary_menu_button("新棋盘", Message::NewBoard))
                (styles::button::secondary_menu_button("添加引擎...", Message::AddEngineButton))
            ))
            )
            (
            text("引擎"),
            menu_template(engine_path).max_width(e_len as f32 * 10.0)
            )
        ).spacing(10.0);
        //colunm
        let engine_table = responsive(|size| {
            let engine_tables = table(
                self.engine_table_info.header.clone(),
                self.engine_table_info.body.clone(),
                &self.engine_table_info.columns,
                &self.engine_table_info.rows,
                Message::EngineTableSyncHeader,
            )
            .footer(self.engine_table_info.footer.clone())
            .min_width(size.width);
            engine_tables.into()
        });
        //board-
        let board = canvas(Board {
            count: self.board_state.chessboard.get_length(),
            pieces: self.board_state.chessboard.get_pieces(),
        })
        .width(Length::Fill)
        .height(Length::Fill);

        let start_index = self.engine_msg.len().saturating_sub(100);
        let engine_output = selection_list::SelectionList::new_with(
            &self.engine_msg[start_index..],
            Message::EngineOutputSelected,
            12.0,
            3.0,
            iced_aw::style::selection_list::primary,
            None,
            Font::with_name("汉仪文黑"),
        )
        .width(250.0);

        let rate = progress_bar(0.0..=100.0, 35.0).style(|_| progress_bar::Style {
            background: Background::Color(Color::WHITE),
            bar: Background::Color(Color::BLACK),
            border: Border::default()
                .color(Color::from_rgb8(211, 211, 211))
                .width(2.0),
        });
        // Render the chessboard and pieces
        let mut main_view = Column::new().push(menu_bar);
        if self.show_engine_manager {
            main_view = main_view.push(engine_table);
        }
        main_view
            .push(row![board, engine_output].spacing(5.0))
            .push(rate)
            .padding(10)
            .spacing(5)
            .into()
    }

    fn title(&self) -> String {
        if let Some(i) = self.engine_path.current_path {
            format!("RBoard - {}", self.engine_path.paths[i as usize].name)
        } else {
            "RBoard".to_string()
        }
    }
}
