use crate::game::scene::map::MapCfg;
use ggez::{graphics, Context};

pub struct PlayerView {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl PlayerView {
    pub fn new(x: u32, y: u32, ctx: &mut Context, map: &MapCfg) -> Self {
        let (width, height) = Self::size(ctx);
        let mut view = Self {
            x: 0,
            y: 0,
            width,
            height,
        };
        view.update(x, y, map);
        view
    }

    pub fn update(&mut self, x: u32, y: u32, map: &MapCfg) {
        let map_width = map.width;
        let map_height = map.height;

        assert!(x < map_width);
        assert!(y < map_height);

        let vx = self.width / 2;
        let vy = self.height / 2;

        let mut x1 = if x <= vx { 0 } else { x - vx };
        let x2 = x1 + self.width;

        if x2 > map_width {
            x1 -= (x2 - map_width).min(x1);
        }

        let mut y1 = if y <= vy { 0 } else { y - vy };
        let y2 = y1 + self.height;

        if y2 > map_height {
            y1 -= (y2 - map_height).min(y1);
        }

        self.x = x1;
        self.y = y1;
    }

    pub fn size(ctx: &Context) -> (u32, u32) {
        let (w, h) = graphics::drawable_size(ctx);
        (w.ceil() as u32, h.ceil() as u32)
    }
}
