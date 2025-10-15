use iced::{
    alignment::{Horizontal, Vertical}, 
    mouse, 
    widget::{button, canvas::{self, Event, Frame, Path, Program}, container, text, Canvas}, 
    Border, Color, Element, Length, Point, Rectangle, Shadow, Size, Task, Vector
};
use iced::widget::canvas::Geometry;
use iced::{Renderer, Theme};
use iced::window;

pub struct Statis {
    selecting: bool,
    start: Option<Point>,
    current: Option<Point>,
}

#[derive(Debug, Clone)]
pub enum Message {
    CaptureFullScreen,
    MousePressed(Point),
    MouseMoved(Point),
    MouseReleased(Point),
}

impl Statis {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                selecting: false,
                start: None,
                current: None,
            }, 
            Task::none()
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::CaptureFullScreen => {
                println!("Entering selection mode!");
                self.selecting = true;
                // Resize window to fullscreen
                Task::batch(vec![
                    window::get_oldest().and_then(|id| window::resize(id, Size::new(1920.0, 1080.0))),
                    window::get_oldest().and_then(|id| window::move_to(id, Point::new(0.0, 0.0)))
                ])
            }
            Message::MousePressed(p) => {
                if self.selecting {
                    self.start = Some(p);
                    self.current = Some(p);
                }
                Task::none()
            }
            Message::MouseMoved(p) => {
                if self.selecting && self.start.is_some() {
                    self.current = Some(p);
                }
                Task::none()
            }
            Message::MouseReleased(p) => {
                if self.selecting {
                    if let Some(start) = self.start {
                        let x1 = start.x.min(p.x);   
                        let y1 = start.y.min(p.y);
                        let x2 = start.x.max(p.x);
                        let y2 = start.y.max(p.y);
                        
                        println!("Selected region: x={} y={} width={} height={}", x1, y1, x2-x1, y2-y1);
                        
                        // TODO: Call your capture code here
                        
                        self.selecting = false;
                        self.start = None;
                        self.current = None;
                        
                        // Restore original window size and position
                        return Task::batch(vec![
                            window::get_oldest().and_then(|id| window::resize(id, Size::new(400.0, 80.0))),
                            window::get_oldest().and_then(|id| window::move_to(id, Point::new(760.0, 0.0)))
                        ]);
                    }
                }
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        if self.selecting {
            let program = SelectionProgram {
                start: self.start,
                current: self.current,
            };
            
            let canvas = Canvas::new(program)
                .width(Length::Fill)
                .height(Length::Fill);
            
            return container(canvas)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(|_| container::Style {
                    background: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.4).into()),
                    ..Default::default()
                })
                .into();
        }

        let btn = button(
            text("Capture Area")
                .size(16)
        )
        .on_press(Message::CaptureFullScreen)
        .padding([10, 24])
        .style(|_theme, status| {
            let base = Color::from_rgba(0.25, 0.25, 0.3, 0.9);
            let hover = Color::from_rgba(0.35, 0.35, 0.45, 0.95);
            button::Style {
                background: Some(match status {
                    button::Status::Hovered => hover.into(),
                    button::Status::Pressed => Color::from_rgba(0.2, 0.2, 0.25, 0.9).into(),
                    _ => base.into(),
                }),
                text_color: Color::WHITE,
                border: Border {
                    color: Color::from_rgba(0.6, 0.6, 0.7, 0.6),
                    width: 1.0,
                    radius: 10.0.into(),
                },
                shadow: Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.5),
                    offset: Vector::new(0.0, 3.0),
                    blur_radius: 10.0,
                },
                ..Default::default()
            }
        });

        container(btn)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(16)
            .center(Length::Fill)
            .align_y(Vertical::Center)
            .style(|_theme| container::Style {
                background: Some(Color::from_rgba(0.12, 0.12, 0.15, 0.95).into()),
                border: Border {
                    color: Color::from_rgba(0.4, 0.4, 0.5, 0.6),
                    width: 1.0,
                    radius: 14.0.into(),
                },
                shadow: Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.7),
                    offset: Vector::new(0.0, 6.0),
                    blur_radius: 16.0,
                },
                ..Default::default()
            })
            .into()
    }
}

struct SelectionProgram {
    start: Option<Point>,
    current: Option<Point>,
}

impl Program<Message> for SelectionProgram {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let Some(pos) = cursor.position_in(bounds) {
                    return (canvas::event::Status::Captured, Some(Message::MousePressed(pos)));
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                return (canvas::event::Status::Captured, Some(Message::MouseMoved(position)));
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                if let Some(pos) = cursor.position_in(bounds) {
                    return (canvas::event::Status::Captured, Some(Message::MouseReleased(pos)));
                }
            }
            _ => {}
        }
        (canvas::event::Status::Ignored, None)
    }

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        
        if let (Some(start), Some(end)) = (self.start, self.current) {
            let x = start.x.min(end.x);
            let y = start.y.min(end.y);
            let w = (end.x - start.x).abs();
            let h = (end.y - start.y).abs();
            
            let rect = Path::rectangle(Point::new(x, y), iced::Size::new(w, h));
            
            // Semi-transparent blue fill
            frame.fill(&rect, Color::from_rgba(0.3, 0.6, 1.0, 0.3));
            
            // Bright border
            frame.stroke(
                &rect,
                canvas::Stroke {
                    style: canvas::Style::Solid(Color::from_rgb(0.3, 0.7, 1.0)),
                    width: 3.0,
                    ..Default::default()
                },
            );
        }
        
        vec![frame.into_geometry()]
    }
}
