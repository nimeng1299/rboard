use iced::Color;

pub trait ChessboardTrait {
    //(winth, height)
    fn get_length(&self) -> (u32, u32);
    //(内圆颜色，外框颜色)
    fn get_pieces(&self) -> Vec<Vec<Option<(Color, Color)>>>;
    //Err -> None
    // Ok -> eg. "play W G3"
    fn go(&mut self, x: i32, y: i32) -> Option<String>;
    fn new_board(&mut self);
    fn get_player(&self) -> Player;
}

#[derive(PartialEq)]
pub enum Player {
    Black,
    White,
}
