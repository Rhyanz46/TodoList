mod implements;
mod app;
mod models;

use iced::{Application, Settings};
use crate::models::TodoApp;

fn main() -> iced::Result {
    TodoApp::run(Settings {
        window: iced::window::Settings {
            size: iced::Size::new(500.0, 600.0),
            min_size: Some(iced::Size::new(400.0, 300.0)),
            ..Default::default()
        },
        ..Default::default()
    })
}