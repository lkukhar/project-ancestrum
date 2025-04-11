use iced::{Application, Settings};
use crate::ui::AncestrumApp;

mod models;
mod storage;
mod ui;

fn main() -> iced::Result {
    AncestrumApp::run(Settings::default())
} 