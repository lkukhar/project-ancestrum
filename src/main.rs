use iced::{Settings, Application};
use crate::ui::ancestrum_app::AncestrumApp;

mod models;
mod storage;
mod ui;

fn main() -> iced::Result {
    AncestrumApp::run(Settings::default())
} 