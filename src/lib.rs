pub mod board;
pub mod chessboard;
pub mod engine;
pub mod message;
pub mod style;

use std::sync::{Arc, Mutex};

use iced::futures::{self, SinkExt, Stream, StreamExt};
use iced::widget::{Column, canvas, column, progress_bar, responsive, row, text};
use iced::{Background, Border, Color, Font, Length, Subscription, Task};
use iced_aw::menu::{Item, Menu};
use iced_aw::{menu_bar, menu_items, selection_list};
use iced_table::table;
use rfd::AsyncFileDialog;

use crate::board::{Board, BoardState};
use crate::engine::analyze::{Analyze, Analyzes};
use crate::engine::analyzes_table::AnalyzesTable;
use crate::engine::engine_paths::EnginePaths;
use crate::engine::engine_table::EngineTable;
use crate::engine::gtp::GTP;
use crate::message::Message;

use crate::style as styles;

pub fn start() -> iced::Result {
    iced::application(RBoard::title, RBoard::update, RBoard::view)
        .subscription(RBoard::subscription)
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

    engine_tx: Arc<Mutex<iced::futures::channel::mpsc::Sender<String>>>,

    engine_analyzes_table: AnalyzesTable,
    analyzes: Arc<Analyzes>,
}

impl Default for RBoard {
    fn default() -> Self {
        let (tx, _) = futures::channel::mpsc::channel::<String>(100);

        Self {
            board_state: Default::default(),
            engine_path: Default::default(),
            show_engine_manager: false,
            engine_table_info: Default::default(),
            engine: None,
            engine_msg: Vec::new(),
            engine_analyze: String::new(),
            engine_tx: Arc::new(Mutex::new(tx)),
            engine_analyzes_table: Default::default(),
            analyzes: Default::default(),
        }
    }
}

impl RBoard {
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::GoBoard(x, y) => {
                if let Some(cmd) = self.board_state.chessboard.go(x, y) {
                    if let Some(gtp) = self.engine.take() {
                        println!("cmd: {}", cmd);
                        let _ = gtp.send_command(cmd);
                        let _ = gtp.send_kata_analyze();
                        self.engine = Some(gtp);
                    }
                }
            }
            Message::NewBoard => {
                self.board_state.chessboard.new_board();
                if let Some(gtp) = self.engine.take() {
                    let _ = gtp.send_command("stop".to_string());
                    let _ = gtp.send_command("clear_board".to_string());
                    let _ = gtp.send_kata_analyze();
                    self.engine = Some(gtp);
                }
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
            Message::EngineSender(sender) => {
                println!("change sender!");
                self.engine_tx = Arc::new(Mutex::new(sender));
            }
            Message::EngineReceiveOutput(data) => {
                if data.starts_with("info") {
                    self.engine_analyze = data;
                    let analyzes = Analyzes::from_string(&self.engine_analyze);
                    self.engine_analyzes_table.rows = analyzes.datas.clone();
                    self.analyzes = Arc::new(analyzes);
                } else {
                    self.engine_msg.push(data);
                }
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
            analyzes: Arc::clone(&self.analyzes),
        })
        .width(Length::Fill)
        .height(Length::Fill);

        //analyze table
        let analyze_table = responsive(|size| {
            let analyze_table = table(
                self.engine_analyzes_table.header.clone(),
                self.engine_analyzes_table.body.clone(),
                &self.engine_analyzes_table.columns,
                &self.engine_analyzes_table.rows,
                Message::EngineTableSyncHeader,
            )
            .min_width(size.width);
            analyze_table.into()
        });

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
            .push(
                row![column![rate, engine_output].spacing(5.0).width(250), board]
                    .spacing(5.0)
                    .height(Length::Fill),
            )
            .push(column![analyze_table].height(100.0))
            .padding(10)
            .spacing(5)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::run(get_data)
    }

    fn title(&self) -> String {
        if let Some(i) = self.engine_path.current_path {
            format!("RBoard - {}", self.engine_path.paths[i as usize].name)
        } else {
            "RBoard".to_string()
        }
    }
}

fn get_data() -> impl Stream<Item = Message> {
    iced::stream::channel(1000, |mut output| async move {
        let (sender, mut receiver) = iced::futures::channel::mpsc::channel::<String>(1000);
        let _ = output.send(Message::EngineSender(sender)).await;
        while let input = receiver.select_next_some().await {
            let _ = output.send(Message::EngineReceiveOutput(input)).await;
        }
    })
}
