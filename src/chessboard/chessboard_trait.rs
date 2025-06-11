use iced::Color;

pub trait ChessboardTrait {
    //(winth, height)
    fn get_length(&self) -> (u32, u32);
    //(内圆颜色，外框颜色)
    fn get_pieces(&self) -> Vec<Vec<(Color, Color)>>;
}
