use std::{f32::consts::PI, time::Instant};

use iced::{
    canvas::{self, Cache, Canvas, Cursor, Geometry, Path, Stroke},
    executor, Application, Color, Command, Element, Length, Point, Rectangle,
    Settings, Subscription,
};

pub fn main() -> iced::Result {
    ArcTo::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

struct ArcTo {
    start: Instant,
    cache: Cache,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick,
}

impl Application for ArcTo {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            ArcTo {
                start: Instant::now(),
                cache: Default::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Arc - Iced")
    }

    fn update(&mut self, _: Message) -> Command<Message> {
        self.cache.clear();
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(10))
            .map(|_| Message::Tick)
    }

    fn view(&mut self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl<Message> canvas::Program<Message> for ArcTo {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let geometry = self.cache.draw(bounds.size(), |frame| {
            let background = Path::rectangle(Point::ORIGIN, frame.size());
            frame.fill(&background, Color::from_rgb8(0x19, 0x19, 0x1D));

            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 5.0;

            let start = Point::new(center.x, center.y - radius);

            let angle = (self.start.elapsed().as_millis() % 10_000) as f32
                / 10_000.0
                * 2.0
                * PI;

            let end = Point::new(
                center.x + radius * angle.cos(),
                center.y + radius * angle.sin(),
            );

            let circles = Path::new(|b| {
                b.circle(start, 10.0);
                b.move_to(end);
                b.circle(end, 10.0);
            });

            frame.fill(&circles, Color::WHITE);

            let path = Path::new(|b| {
                b.move_to(start);
                b.arc_to(center, end, 50.0);
                b.line_to(end);
            });

            frame.stroke(
                &path,
                Stroke {
                    color: Color::WHITE,
                    width: 10.0,
                    ..Stroke::default()
                },
            );
        });

        vec![geometry]
    }
}
