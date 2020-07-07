use crate::position::Position;
use ggez::graphics;
use ggez::graphics::DrawParam;
use ggez::graphics::Image;
use ggez::Context;
use ggez::GameResult;
use std::rc::Rc;
use std::time::Instant;

mod cfg;
pub use cfg::*;

pub struct Tank {
    cfg: Rc<TankCfg>,
    position: Position,
}

impl Tank {
    pub fn new(cfg: Rc<TankCfg>, x: u32, y: u32) -> Tank {
        Tank {
            cfg,
            position: Position::new(x, y),
        }
    }

    pub fn move_to(&mut self, x: u32, y: u32, now: Instant) {
        self.position.move_to(x, y, self.cfg.ms, now);
    }

    pub fn update(&mut self, now: Instant) -> bool {
        self.position.update(now)
    }

    pub fn draw(&mut self, ctx: &mut Context, x1: u32, y1: u32, flag: &Image) -> GameResult {
        let dx = self.x() as f64 - x1 as f64;
        let dy = self.y() as f64 - y1 as f64;

        // self.batch.add(
        //     DrawParam::new()
        //         .dest([dx as f32, dy as f32])
        //         .offset([0.5, 0.5]),
        // );

        graphics::draw(
            ctx,
            &self.cfg.image,
            DrawParam::new()
                .dest([dx as f32, dy as f32])
                .offset([0.5, 0.5]),
        )?;
        // self.batch.clear();

        if let Some((fx, fy)) = self.position.destination() {
            let fx = fx as f64 - x1 as f64;
            let fy = fy as f64 - y1 as f64;

            graphics::draw(
                ctx,
                flag,
                DrawParam::new()
                    .dest([fx as f32, fy as f32])
                    .offset([0.5, 1.0]),
            )?;
        }

        Ok(())
    }

    pub fn x(&self) -> u32 {
        self.position.x()
    }

    pub fn y(&self) -> u32 {
        self.position.y()
    }
}
