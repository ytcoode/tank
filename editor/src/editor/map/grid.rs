use crate::editor::Msg;
use iced::canvas::{self, Cache, Canvas, Cursor, Event, Geometry};
use iced::mouse;
use iced::widget::image::Image;
use iced::{Color, Element, Length, Point, Rectangle, Size};
use std::convert::TryInto;

pub struct Grid {
    tiles: Vec<u8>,
    tile_rows: u32,
    tile_cols: u32,
    tile_size: u32,
    tile_images: Vec<Image>,
    cache: Cache,
}

impl Grid {
    pub fn new(rows: u32, cols: u32) -> Grid {
        let tiles = Vec::with_capacity((rows * cols).try_into().unwrap());
        let mut tile_images = Vec::new();

        let image = Image::new("assets/resources/a/PNG/Environment/dirt.png");
        tile_images.push(image);

        let image = Image::new("assets/resources/a/PNG/Environment/grass.png");
        tile_images.push(image);

        let image = Image::new("assets/resources/a/PNG/Environment/sand.png");
        tile_images.push(image);

        let tile_size = 128; // There is no way to get the size of tile images, so we hardcode it here.

        Grid {
            tiles,
            tile_rows: rows,
            tile_cols: cols,
            tile_size,
            tile_images,
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

            let tile_size = self.tile_size.try_into().unwrap();

            for x in (0..width as u32).step_by(tile_size) {
                frame.fill_rectangle(Point::new(x as f32, 0.0), Size::new(1.0, height), color);
            }

            for y in (0..height as u32).step_by(tile_size) {
                frame.fill_rectangle(Point::new(0.0, y as f32), Size::new(width, 1.0), color);
            }
        });

        vec![grid]
    }

    fn mouse_interaction(&self, _bounds: Rectangle, _cursor: Cursor) -> mouse::Interaction {
        mouse::Interaction::default()
    }
}
