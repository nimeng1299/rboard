#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rboard::start;

fn main() -> iced::Result {
    start()
}
