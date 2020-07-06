use crate::util;
use ggez::graphics;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::DrawParam;
use ggez::graphics::Image;
use ggez::Context;
use ggez::GameResult;

mod cfg;
pub use cfg::*;

pub struct Tank {
    pub cfg: TankCfg,

    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,

    pub x1: f32,
    pub y1: f32,

    pub x0: f32,
    pub y0: f32,
    pub dx: f32,
    pub dy: f32,
}

impl Tank {
    pub fn new(cfg: TankCfg) -> Tank {
        let batch = SpriteBatch::new(image);

        Ok(Tank {
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            x1: 0.0,
            y1: 0f32,
            dx: 0.0,
            dy: 0.0,
            x0: 0.0,
            y0: 0.0,
        })
    }

    pub fn draw(&mut self, ctx: &mut Context, x1: u32, y1: u32) -> GameResult {
        let x = self.x - x1 as f32;
        let y = self.y - y1 as f32;

        self.batch
            .add(DrawParam::new().dest([x, y]).offset([0.5, 0.5]));

        graphics::draw(ctx, &self.batch, util::DRAW_PARAM_ZERO)?;
        self.batch.clear();

        Ok(())
    }
}
