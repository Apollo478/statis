
use crate::statis_ui::Statis;
use iced::Result;
mod statis_ui;

mod capture;
fn main() -> iced::Result {
    iced::application("Statis", Statis::update, Statis::view)
            .window_size((400.0, 80.0))
            .position(iced::window::Position::Specific(iced::Point::new(760.0, 0.0)))
            .transparent(true)
            .decorations(false)
            .run_with(Statis::new)
}
