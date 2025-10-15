
use crate::statis_ui::Statis;
use iced::Result;
mod statis_ui;

mod capture;
fn main() -> iced::Result {
    iced::application("Statis", Statis::update, Statis::view)
        .window_size((420.0, 80.0))
        .transparent(true)
        .decorations(false)
        .position(iced::window::Position::Specific(iced::Point { x: 760.0, y: 0.0 }))
        .run_with(Statis::new)
}
