use crate::util;
use ggez::graphics;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::Image;
use ggez::Context;
use ggez::GameResult;

pub struct Tank {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub batch: SpriteBatch,
}

impl Tank {
    pub fn new(ctx: &mut Context) -> GameResult<Tank> {
        let tank_texture = Image::new(ctx, "/PNG/Tanks/tankGreen.png")?;
        let batch = SpriteBatch::new(tank_texture);

        Ok(Tank {
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            batch,
        })
    }

    pub fn draw(&mut self, ctx: &mut Context, x1: u32, y1: u32) -> GameResult {
        let x = self.x as f32 - x1 as f32;
        let y = self.y as f32 - y1 as f32;
        self.batch.add(([x, y],));
        graphics::draw(ctx, &self.batch, util::DRAW_PARAM_ZERO)?;
        self.batch.clear();
        Ok(())
    }
}
