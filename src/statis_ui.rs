use iced::{
    widget::{button, container, text, Column},
    alignment::{Horizontal, Vertical},
    Color, Element, Task, Border, Shadow, Subscription, Length, Vector,

};

pub struct Statis {
    drop_progress: f32,
}

#[derive(Debug, Clone)]
pub enum Message {
    CaptureFullScreen,
    Tick,
}

impl Statis {
    pub fn new() -> (Self, Task<Message>) {
        (Self { drop_progress: 0.0 }, Task::none())
    }


    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::CaptureFullScreen => {
                println!("Capturing screenshot!");
            }
            Message::Tick => {
                if self.drop_progress < 1.0 {
                    self.drop_progress += 0.05;
                }
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<Message> {
        let drop_offset = -100.0 + (self.drop_progress * 100.0);

        let btn = button(
            text("Capture Full Screen")
                .size(16)
                .style(|_| text::Style {
                    color: Some(Color::from_rgb(0.95, 0.95, 1.0)),
                }),
        )
        .on_press(Message::CaptureFullScreen)
        .padding([10, 24])
        .style(|_theme, status| {
            let base = Color::from_rgba(0.25, 0.25, 0.3, 0.7);
            let hover = Color::from_rgba(0.35, 0.35, 0.45, 0.8);
            button::Style {
                background: Some(match status {
                    button::Status::Hovered => hover.into(),
                    button::Status::Pressed => Color::from_rgba(0.2, 0.2, 0.25, 0.7).into(),
                    _ => base.into(),
                }),
                text_color: Color::WHITE,
                border: Border {
                    color: Color::from_rgba(0.6, 0.6, 0.7, 0.5),
                    width: 1.0,
                    radius: 10.0.into(),
                },
                shadow: Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.4),
                    offset: Vector::new(0.0, 3.0),
                    blur_radius: 10.0,
                },
                ..Default::default()
            }
        });

        let layout = Column::new()
            .push(btn)
            .align_x(Horizontal::Center)
            .spacing(10)
            .width(Length::Fill)
            .height(Length::Fill);

        container(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(16)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .style(move |_theme| container::Style {
                background: Some(Color::from_rgba(0.12, 0.12, 0.15, 0.9).into()),
                border: Border {
                    color: Color::from_rgba(0.4, 0.4, 0.5, 0.5),
                    width: 1.0,
                    radius: 14.0.into(),
                },
                shadow: Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.6),
                    offset: Vector::new(0.0, 6.0 + drop_offset / 10.0),
                    blur_radius: 16.0,
                },
                ..Default::default()
            })
            .into()
    }
}


