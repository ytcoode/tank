use crate::util;
use ggez::graphics;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::DrawParam;
use ggez::graphics::Image;
use ggez::Context;
use ggez::GameResult;
use std::rc::Rc;

mod cfg;
pub use cfg::*;

pub struct Tank {
    pub cfg: Rc<TankCfg>,
    pub x: u32,
    pub y: u32,
}

impl Tank {
    pub fn new(cfg: Rc<TankCfg>) -> Tank {
        Tank { cfg, x: 0, y: 0 }
    }

    pub fn draw(&mut self, ctx: &mut Context, x1: u32, y1: u32) -> GameResult {
        // let x = self.x - x1 as f32;
        // let y = self.y - y1 as f32;

        // self.batch
        //     .add(DrawParam::new().dest([x, y]).offset([0.5, 0.5]));

        // graphics::draw(ctx, &self.batch, util::DRAW_PARAM_ZERO)?;
        // self.batch.clear();

        Ok(())
    }
}
