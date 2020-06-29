use crate::debug;
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

    pub width: u32,
    pub height: u32,
    pub batch: SpriteBatch,
}

impl Tank {
    pub fn new(ctx: &mut Context) -> GameResult<Tank> {
        let image = Image::new(ctx, "/PNG/Tanks/tankRed.png")?;
        let width = image.width() as u32;
        let height = image.height() as u32;
        let batch = SpriteBatch::new(image);

        Ok(Tank {
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            width,
            height,
            batch,
        })
    }

    pub fn draw(&mut self, ctx: &mut Context, x1: u32, y1: u32) -> GameResult {
        let x1 = x1 as f32;
        let y1 = y1 as f32;

        let x = self.x - x1;
        let y = self.y - y1;
        self.batch.add(([x, y],));

        graphics::draw(ctx, &self.batch, util::DRAW_PARAM_ZERO)?;
        self.batch.clear();

        Ok(())
    }
}
