use super::map::Map;
use ggez::graphics;
use ggez::Context;

#[derive(Default)]
pub struct Vision {
    pub x1: u32,
    pub x2: u32,
    pub y1: u32,
    pub y2: u32,
    width: u32,
    height: u32,
}

impl Vision {
    pub fn new(x: u32, y: u32, map: &Map, ctx: &mut Context) -> Vision {
        let mut r = Vision::default();
        r.resize(x, y, map, ctx);
        r
    }

    pub fn resize(&mut self, x: u32, y: u32, map: &Map, ctx: &mut Context) {
        let (vw, vh) = graphics::drawable_size(ctx);
        self.width = vw.ceil() as u32;
        self.height = vh.ceil() as u32;
        self.update(x, y, map);
    }

    pub fn update(&mut self, x: u32, y: u32, map: &Map) {
        let map_width = map.width();
        let map_height = map.height();

        assert!(x < map_width);
        assert!(y < map_height);

        let vx = self.width / 2;
        let vy = self.height / 2;

        let mut x1 = if x <= vx { 0 } else { x - vx };
        let mut x2 = x1 + self.width;

        if x2 > map_width {
            x1 -= (x2 - map_width).min(x1);
            x2 = x1 + self.width;
        }

        let mut y1 = if y <= vy { 0 } else { y - vy };
        let mut y2 = y1 + self.height;

        if y2 > map_height {
            y1 -= (y2 - map_height).min(y1);
            y2 = y1 + self.height;
        }

        self.x1 = x1;
        self.x2 = x2;
        self.y1 = y1;
        self.y2 = y2;
    }
}
