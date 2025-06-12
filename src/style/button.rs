use iced::{
    Background, Border, Color, Element, Length,
    widget::{
        self,
        button::{self, Status, Style},
    },
};

use crate::message::Message;

pub fn secondary_menu_button<'a>(
    content: impl Into<Element<'a, Message>>,
    msg: Message,
) -> button::Button<'a, Message> {
    widget::button(content)
        // 根据需要调整内边距、最小宽高等
        .padding(iced::Padding {
            left: 20.0,
            right: 10.0,
            top: 5.0,
            bottom: 5.0,
        })
        .width(Length::Fill) // 二级菜单通常撑满父宽度
        .style(|theme, status| {
            let style = button::secondary(theme, status);
            match status {
                Status::Active | Status::Pressed => Style {
                    background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.0))),
                    ..style
                },
                Status::Hovered => Style {
                    border: Border::default()
                        .width(1.0)
                        .rounded(3.0)
                        .color(Color::from_rgb8(211, 211, 211)),
                    ..style
                },
                _ => style,
            }
        })
        .on_press(msg)
}
