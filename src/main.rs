use iced::{Sandbox, Settings};
use crate::ui::ancestrum_app::AncestrumApp;

mod models;
mod storage;
mod ui;

fn main() -> iced::Result {
    AncestrumApp::run(Settings::default())
} 