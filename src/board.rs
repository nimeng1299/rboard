use std::cmp::max;

use iced::{
    Color, Point, Renderer, Theme,
    mouse::{self, Cursor},
    widget::canvas::{self, Geometry, Stroke, Text},
};

use crate::{
    chessboard::{chessboard_trait::ChessboardTrait, get_chessboard},
    message::Message,
};

pub struct Board {
    pub count: (u32, u32),
    pub pieces: Vec<Vec<Option<(Color, Color)>>>,
}

impl canvas::Program<Message> for Board {
    type State = ();

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: iced::Rectangle,
        cursor: Cursor,
    ) -> Vec<Geometry<Renderer>> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        frame.fill_rectangle(
            Point::ORIGIN,
            bounds.size(),
            Color::from_rgb8(247, 238, 214),
        );

        let (x, y) = self.count;
        let x_size = (bounds.width - 5f32) / (x + 1) as f32;
        let y_size = (bounds.height - 5f32) / (y + 1) as f32;
        let size = x_size.min(y_size);

        let x_padding = (bounds.width - size * x as f32) / 2f32 + size / 2f32;
        let y_padding = (bounds.height - size * y as f32) / 2f32 + size / 2f32;

        for i in 0..=x {
            let start = Point::new(x_padding + (i as f32) * size, y_padding);
            let end = Point::new(x_padding + (i as f32) * size, (y as f32) * size + y_padding);
            let line = canvas::Path::line(start, end);
            frame.stroke(
                &line,
                Stroke::default()
                    .with_width(1.0f32)
                    .with_color(Color::BLACK),
            );
        }

        for i in 0..=y {
            let start = Point::new(x_padding, y_padding + (i as f32) * size);
            let end = Point::new((x as f32) * size + x_padding, y_padding + (i as f32) * size);
            let line = canvas::Path::line(start, end);
            frame.stroke(
                &line,
                Stroke::default()
                    .with_width(1.0f32)
                    .with_color(Color::BLACK),
            );
        }

        for i in 0..x {
            let label = (b'A' + i as u8) as char;

            let label_size = match label {
                'i' | 'I' | 'J' => 0.2,
                'm' | 'w' | 'M' => 0.7,
                _ => 0.5,
            };
            let label = label.to_string();

            let position = Point {
                x: x_padding + i as f32 * size + (size * (1f32 - label_size)) / 2f32,
                y: y_padding - size,
            };
            frame.fill_text(Text {
                content: label,
                position,
                color: Color::BLACK,
                size: iced::Pixels(size * 0.8),
                ..Default::default()
            });
        }

        for j in 0..y {
            let mut label = (j + 1).to_string();
            if j < 9 {
                label = " ".to_owned() + &label;
            }
            let position = Point {
                x: x_padding - size,
                y: y_padding + (y - 1 - j) as f32 * size,
            };
            frame.fill_text(Text {
                content: label,
                position,
                color: Color::BLACK,
                size: iced::Pixels(size * 0.8),
                ..Default::default()
            });
        }

        //鼠标位置
        let p = cursor.position_in(bounds);
        if let Some(p_cursor) = p {
            let x_count = ((p_cursor.x - x_padding) / size).floor() as i32;
            let y_count = ((p_cursor.y - y_padding) / size).floor() as i32;
            if x_count >= 0 && x_count < x as i32 && y_count >= 0 && y_count < y as i32 {
                // 定义颜色
                let light_gray = Color::from_rgb(0.9, 0.9, 0.9); // 淡灰色背景
                let purple = Color::from_rgb(0.5, 0.0, 0.5); // 紫色边框

                let radius = size / 2.0;
                // 设置圆的位置和大小
                let center = iced::Point::new(
                    x_padding + x_count as f32 * size + radius,
                    y_padding + y_count as f32 * size + radius,
                );
                let circle = canvas::Path::circle(center, radius * 0.8);

                frame.fill(&circle, light_gray);

                frame.stroke(
                    &circle,
                    Stroke::default().with_color(purple).with_width(2.0),
                );
            }
        }

        //画棋子
        for i in 0..self.pieces.len() {
            for j in 0..self.pieces[i].len() {
                if let Some((c1, c2)) = self.pieces[i][j] {
                    let x = x_padding + i as f32 * size + size / 2.0;
                    let y = y_padding + j as f32 * size + size / 2.0;
                    let center = iced::Point::new(x, y);
                    let circle = canvas::Path::circle(center, size / 2.0 * 0.9);

                    frame.fill(&circle, c1);

                    frame.stroke(&circle, Stroke::default().with_color(c2).with_width(2.0));
                }
            }
        }
        vec![frame.into_geometry()]
    }

    fn update(
        &self,
        _state: &mut Self::State,
        _event: canvas::Event,
        _bounds: iced::Rectangle,
        _cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        match _event {
            canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                let (x, y) = self.count;
                let x_size = (_bounds.width - 5f32) / (x + 1) as f32;
                let y_size = (_bounds.height - 5f32) / (y + 1) as f32;
                let size = x_size.min(y_size);

                let x_padding = (_bounds.width - size * x as f32) / 2f32 + size / 2f32;
                let y_padding = (_bounds.height - size * y as f32) / 2f32 + size / 2f32;
                let p = _cursor.position_in(_bounds);
                if let Some(p_cursor) = p {
                    let x_count = ((p_cursor.x - x_padding) / size).floor() as i32;
                    let y_count = ((p_cursor.y - y_padding) / size).floor() as i32;
                    if x_count >= 0 && x_count < x as i32 && y_count >= 0 && y_count < y as i32 {
                        return (
                            canvas::event::Status::Ignored,
                            Some(Message::GoBoard(x_count, y_count)),
                        );
                    }
                }
            }
            _ => {}
        }
        (canvas::event::Status::Ignored, None)
    }
}

pub struct BoardState {
    pub chessboard: Box<dyn ChessboardTrait>,
}

impl Default for BoardState {
    fn default() -> Self {
        BoardState {
            chessboard: get_chessboard("".to_string()),
        }
    }
}
