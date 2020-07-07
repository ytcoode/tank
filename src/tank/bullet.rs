use ggez::graphics;
use ggez::graphics::DrawParam;
use ggez::graphics::Image;
use ggez::Context;
use ggez::GameResult;
use std::time::Instant;

pub struct Bullet {
    sx: u32,
    sy: u32,
    cx: u32,
    cy: u32,
    angle: f64,
    st: Instant,
}

impl Bullet {
    pub fn new(x: u32, y: u32, angle: f64, st: Instant) -> Bullet {
        Bullet {
            sx: x,
            sy: y,
            cx: x,
            cy: y,
            angle,
            st,
        }
    }

    pub fn update(&mut self, now: Instant) {
        let dt = now.duration_since(self.st).as_secs_f64();
        let distance = dt * 200.0;
        let dx = self.angle.cos() * distance;
        let dy = self.angle.sin() * distance;
        self.cx = (self.sx as f64 + dx) as u32;
        self.cy = (self.sy as f64 + dy) as u32;
    }

    pub fn draw(&mut self, ctx: &mut Context, x1: u32, y1: u32, bullet: &Image) -> GameResult {
        let dx = self.cx as f64 - x1 as f64;
        let dy = self.cy as f64 - y1 as f64;

        graphics::draw(
            ctx,
            bullet,
            DrawParam::new()
                .dest([dx as f32, dy as f32])
                .offset([0.5, 0.5])
                .rotation(self.angle as f32),
        )?;

        Ok(())
    }
}
