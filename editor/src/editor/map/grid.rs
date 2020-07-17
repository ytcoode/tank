use crate::editor::Msg;
use iced::canvas::{self, Cache, Canvas, Cursor, Event, Geometry};
use iced::mouse;
use iced::widget::image::Image;
use iced::{Color, Element, Length, Point, Rectangle, Size};
use std::convert::TryInto;
use util;

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
        let mut tile_images = Vec::new();

        let image = Image::new("assets/resources/a/PNG/Environment/dirt.png");
        tile_images.push(image);

        let image = Image::new("assets/resources/a/PNG/Environment/grass.png");
        tile_images.push(image);

        let image = Image::new("assets/resources/a/PNG/Environment/sand.png");
        tile_images.push(image);

        Grid {
            tiles: Vec::with_capacity((rows * cols).try_into().unwrap()),
            tile_rows: rows,
            tile_cols: cols,
            tile_size: 128, // There is no way to get the size of tile images, so we hardcode it here.
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

    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let grid = self.cache.draw(bounds.size(), |frame| {
            let width = frame.width().ceil() as u32;
            let height = frame.height().ceil() as u32;

            let rows = util::div_ceil(height, self.tile_size).min(self.tile_rows);
            let cols = util::div_ceil(width, self.tile_size).min(self.tile_cols);

            let width = cols * self.tile_size;
            let height = rows * self.tile_size;

            let color = Color::from_rgb8(70, 74, 83);

            let size = Size::new(width as f32, 1.0);
            (0..=rows)
                .map(|r| r * self.tile_size)
                .map(|y| Point::new(0.0, y as f32))
                .for_each(|p| frame.fill_rectangle(p, size, color));

            let size = Size::new(1.0, height as f32);
            (0..=cols)
                .map(|c| c * self.tile_size)
                .map(|x| Point::new(x as f32, 0.0))
                .for_each(|p| frame.fill_rectangle(p, size, color));
        });

        vec![grid]
    }

    fn mouse_interaction(&self, _bounds: Rectangle, _cursor: Cursor) -> mouse::Interaction {
        mouse::Interaction::default()
    }
}
