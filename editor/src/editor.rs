use crate::style;
use grid::Grid;
use iced::{
    canvas::Canvas, executor, Application, Column, Command, Container, Element, Length, Settings,
    Text,
};

pub fn start() {
    Editor::run(Settings::default());
}

struct Editor {
    grid: Grid,
}

#[derive(Debug)]
enum Msg {
    A,
}

impl Application for Editor {
    type Executor = executor::Default;
    type Message = Msg;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (Editor { grid: Grid::new() }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Map Editor")
    }

    fn update(&mut self, _message: Msg) -> Command<Msg> {
        Command::none()
    }

    fn view(&mut self) -> Element<Msg> {
        let t = Text::new("Hello, world!");
        let c = Column::new().push(self.grid.view()).push(t);

        Container::new(c)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style::Container)
            .into()
    }
}

mod grid {
    use super::Msg;

    use iced::{
        canvas::{self, Cache, Canvas, Cursor, Event, Frame, Geometry, Path, Text},
        mouse, Color, Element, HorizontalAlignment, Length, Point, Rectangle, Size, Vector,
        VerticalAlignment,
    };

    const CELL_SIZE: usize = 50;

    pub struct Grid {
        cache: Cache,
    }

    impl Grid {
        pub fn new() -> Grid {
            Grid {
                cache: Cache::default(),
            }
        }

        pub fn view(&mut self) -> Element<'_, Msg> {
            Canvas::new(self)
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        }
    }

    impl canvas::Program<Msg> for Grid {
        fn update(&mut self, _event: Event, _bounds: Rectangle, _cursor: Cursor) -> Option<Msg> {
            None
        }

        fn draw(&self, bounds: Rectangle, cursor: Cursor) -> Vec<Geometry> {
            let grid = self.cache.draw(bounds.size(), |frame| {
                let color = Color::from_rgb8(70, 74, 83);

                let width = frame.width();
                let height = frame.height();

                for x in (0..width as u32).step_by(CELL_SIZE) {
                    frame.fill_rectangle(Point::new(x as f32, 0.0), Size::new(1.0, height), color);
                }

                for y in (0..height as u32).step_by(CELL_SIZE) {
                    frame.fill_rectangle(Point::new(0.0, y as f32), Size::new(width, 1.0), color);
                }
            });

            vec![grid]
        }

        fn mouse_interaction(&self, _bounds: Rectangle, _cursor: Cursor) -> mouse::Interaction {
            mouse::Interaction::default()
        }
    }
}
