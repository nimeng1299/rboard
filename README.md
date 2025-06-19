# rboard
## 程序下载
### 本体下载 [下载地址](https://github.com/nimeng1299/rboard/releases)
### 引擎下载 [由 @hzyhhzy 提供](https://github.com/hzyhhzy/KataGomo)

## 如何添加棋盘
### 1. 创建新的棋盘模块
在 `src/chessboard` 目录下创建一个新的 Rust 模块文件，例如 `new_board.rs`。

### 2. 实现棋盘逻辑
在 `new_board.rs` 中实现新的棋盘逻辑，通过`ChessboardTrait`实现。
> **⚠️ Warning:** 这个Trait未来可能会发生改变
```
fn get_length(&self) -> (u32, u32);
```
返回一个元组，表示棋盘的宽度和高度。
```
fn get_pieces(&self) -> Vec<Vec<Option<(Color, Color)>>>;
```
返回一个二维向量表示棋盘每个位置的棋子状态
```
fn go(&mut self, x: i32, y: i32) -> Option<String>;
```
尝试在指定坐标 `(x, y)` 下棋,下棋成功，返回格式化的落子信息，例如 `"play W G3"`。用于引擎走子。
```
fn new_board(&mut self);
```
重置棋盘，开始新的一局。
```
fn get_player(&self) -> Player;
```
返回当前轮到的玩家（例如黑方或白方）。
### 3. 注册新的棋盘类型
在 `src/chessboard/mod.rs` 中注册新的棋盘类型，以便在程序中使用。
#### 3.1 修改新棋盘类型
```
pub fn get_chessboard(name: String) -> Box<dyn ChessboardTrait> {
    match name.as_str() {
        "gomoku" => Box::new(gomoku::Gomoku::new()),
        "zhenqi" => Box::new(zhenqi::Zhenqi::new()),
        "new_board" => Box::new(new_board::NewBoard::new()),
        _ => Box::new(gomoku::Gomoku::new()),
    }
}
```
`new_board`为棋盘的唯一id，返回一个棋盘的创建函数
#### 3.2 修改棋盘集合
```
pub fn get_all_board_names() -> Vec<(String, String)> {
    vec![
        ("Gomoku 15 * 15".to_string(), "gomoku".to_string()),
        ("Zhenqi 8 * 8".to_string(), "zhenqi".to_string()),
        ("New Board".to_string(), "new_board".to_string()),
    ]
}
```
`New Board`棋盘显示的名字，不唯一，`new_board`为棋盘的唯一id
