mod models;
mod storage;
mod ui;

use ui::AncestrumApp;

fn main() -> iced::Result {
    AncestrumApp::run(Settings::default())
} 