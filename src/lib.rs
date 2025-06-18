pub mod board;
pub mod chessboard;
pub mod engine;
pub mod message;
pub mod style;

use std::sync::{Arc, Mutex};

use iced::futures::{self, SinkExt, Stream, StreamExt};
use iced::widget::{
    Column, button, canvas, column, progress_bar, responsive, row, text, text_editor, text_input,
};
use iced::{Background, Border, Color, Font, Length, Subscription, Task};
use iced_aw::menu::{Item, Menu};
use iced_aw::{SelectionList, menu_bar, menu_items, selection_list};
use iced_table::table;
use rfd::AsyncFileDialog;

use crate::board::{Board, BoardState};
use crate::chessboard::chessboard_trait::Player;
use crate::chessboard::get_all_board_names;
use crate::engine::analyze::{Analyze, Analyzes};
use crate::engine::analyzes_table::AnalyzesTable;
use crate::engine::engine_paths::EnginePaths;
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
    engine_name_list: Vec<String>,
    engine_setting_selected: Option<usize>,
    engine_setting_arg_content: text_editor::Content,

    engine: Option<GTP>,
    engine_msg: Vec<String>,
    engine_analyze: String,

    engine_tx: Arc<Mutex<iced::futures::channel::mpsc::Sender<String>>>,

    engine_analyzes_table: AnalyzesTable,
    analyzes: Arc<Analyzes>,
    black_winrate: f64,
}

impl Default for RBoard {
    fn default() -> Self {
        let (tx, _) = futures::channel::mpsc::channel::<String>(100);

        Self {
            board_state: Default::default(),
            engine_path: Default::default(),
            show_engine_manager: false,
            engine_name_list: Vec::new(),
            engine_setting_selected: None,
            engine_setting_arg_content: Default::default(),
            engine: None,
            engine_msg: Vec::new(),
            engine_analyze: String::new(),
            engine_tx: Arc::new(Mutex::new(tx)),
            engine_analyzes_table: Default::default(),
            analyzes: Default::default(),
            black_winrate: 50.0,
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
                    self.analyzes = Arc::new(Default::default());
                    self.engine_analyzes_table.rows = vec![];
                    self.engine_msg.clear();
                }
            }
            Message::ChangeBoard(name) => {
                self.board_state.change_board(name);
                if let Some(mut gtp) = self.engine.take() {
                    let _ = gtp.exit();
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
                    self.engine_name_list = self
                        .engine_path
                        .paths
                        .iter()
                        .map(|e| e.name.clone())
                        .collect::<Vec<String>>();
                }
                None => {
                    eprintln!("Error adding engine path");
                }
            },
            Message::ChangeEngineSettingSelectionList(i, _) => {
                self.engine_setting_selected = Some(i);
                let s = self.engine_path.paths[i].clone();
                self.engine_setting_arg_content = text_editor::Content::with_text(&s.args);
            }
            Message::OpenEngineManager => {
                self.engine_name_list = self
                    .engine_path
                    .paths
                    .iter()
                    .map(|e| e.name.clone())
                    .collect::<Vec<String>>();
                self.engine_setting_arg_content = text_editor::Content::with_text("");
                self.show_engine_manager = true;
            }
            Message::CloseEngineManager => self.show_engine_manager = false,
            Message::ChangeEngineName(index, name) => {
                let _ = self.engine_path.change_name(index, name);
                self.engine_name_list = self
                    .engine_path
                    .paths
                    .iter()
                    .map(|e| e.name.clone())
                    .collect::<Vec<String>>();
            }
            Message::ChangeEngineArgs(index, action) => {
                self.engine_setting_arg_content.perform(action.clone());
                if let text_editor::Action::Edit(_) = action {
                    let _ = self
                        .engine_path
                        .change_args(index, self.engine_setting_arg_content.text());
                }
            }
            Message::DeleteEngine => {
                if let Some(i) = self.engine_setting_selected {
                    let _ = self.engine_path.delete(i);
                    self.engine_setting_selected = None;
                }
            }
            Message::ChangeEngine(index) => {
                let args = self.engine_path.get_all_paths()[index].clone();

                if let Some(mut engine) = self.engine.take() {
                    let _ = engine.exit();
                }

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
                    if self.analyzes.datas.len() > 0 {
                        let winrate = self.analyzes.datas[0].winrate * 100.0;
                        if self.board_state.chessboard.get_player() == Player::Black {
                            self.black_winrate = winrate;
                        } else {
                            self.black_winrate = 100.0 - winrate;
                        }
                    }
                } else if data.starts_with("Why you give a finished board here") {
                    if let Some(gtp) = self.engine.take() {
                        let _ = gtp.send_command("stop".to_string());
                        self.engine = Some(gtp);
                        self.analyzes = Arc::new(Default::default());
                        self.engine_analyzes_table.rows = vec![];
                    }
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
        engine_path.push(Item::new(styles::button::secondary_menu_button(
            "添加引擎",
            Message::AddEngineButton,
        )));

        let mut all_board = vec![];
        for (name, id) in get_all_board_names() {
            all_board.push(Item::new(styles::button::secondary_menu_button(
                text(name),
                Message::ChangeBoard(id),
            )));
        }

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
            text("棋盘"),
            menu_template(all_board).max_width(300.0)
            )
            (
            text("引擎"),
            menu_template(engine_path).max_width(e_len as f32 * 10.0)
            )
        ).spacing(10.0);

        //engine setting
        let engine_setting_selection_list = SelectionList::new_with(
            &self.engine_name_list[..],
            Message::ChangeEngineSettingSelectionList,
            12.0,
            5.0,
            iced_aw::style::selection_list::primary,
            self.engine_setting_selected,
            Font::with_name("汉仪文黑"),
        )
        .height(70.0);
        let engine_setting_delete_button = button("删除")
            .on_press(Message::DeleteEngine)
            .height(30.0)
            .width(Length::Fill);
        let engine_setting_close_button = button("完成")
            .on_press(Message::CloseEngineManager)
            .height(30.0)
            .width(Length::Fill);
        let engine_setting_name;
        let engine_setting_arg;
        if let Some(i) = self.engine_setting_selected {
            let engine_arg = self.engine_path.paths[i].clone();
            engine_setting_name = text_input("", &engine_arg.path)
                .on_input(move |name| Message::ChangeEngineName(i, name));
            engine_setting_arg = text_editor(&self.engine_setting_arg_content)
                .on_action(move |action| Message::ChangeEngineArgs(i, action));
        } else {
            engine_setting_name = text_input("", "");
            engine_setting_arg = text_editor(&self.engine_setting_arg_content);
        }
        let engine_setting = row![
            column![
                engine_setting_selection_list,
                row![engine_setting_delete_button, engine_setting_close_button]
                    .spacing(2.0)
                    .width(Length::Fill)
            ]
            .width(130.0)
            .spacing(3.0),
            column![engine_setting_name, engine_setting_arg].spacing(3.0)
        ]
        .spacing(5.0)
        .height(100.0);

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

        let rate =
            progress_bar(0.0..=100.0, self.black_winrate as f32).style(|_| progress_bar::Style {
                background: Background::Color(Color::WHITE),
                bar: Background::Color(Color::BLACK),
                border: Border::default()
                    .color(Color::from_rgb8(211, 211, 211))
                    .width(2.0),
            });
        // Render the chessboard and pieces
        let mut main_view = Column::new().push(menu_bar);
        if self.show_engine_manager {
            main_view = main_view.push(engine_setting);
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
