use crate::path::Path;
use ggez::graphics;
use ggez::graphics::DrawParam;
use ggez::graphics::Image;
use ggez::Context;
use ggez::GameResult;
use std::f64;
use std::rc::Rc;
use std::time::Instant;

mod cfg;
pub use cfg::*;

pub struct Tank {
    cfg: Rc<TankCfg>,
    x: u32,
    y: u32,
    path: Option<Path>,
    angle: f32,
    barrel_angle: f32,
    pub barrel_rotation: f32,
}

impl Tank {
    pub fn new(cfg: Rc<TankCfg>, x: u32, y: u32) -> Tank {
        Tank {
            cfg,
            x,
            y,
            path: None,
            angle: 0.0,
            barrel_angle: 0.0,
            barrel_rotation: 0.0,
        }
    }

    pub fn move_to(&mut self, x: u32, y: u32, now: Instant) {
        let path = Path::new(self.x, self.y, x, y, self.cfg.ms, now);
        let angle = path.angle() + f64::consts::FRAC_PI_2;
        self.path = Some(path);
        self.angle = angle as f32;
    }

    pub fn update(&mut self, now: Instant) -> bool {
        self.barrel_angle += self.barrel_rotation;

        match self.path {
            Some(ref p) => {
                let (x, y) = p.position(now);
                self.x = x;
                self.y = y;
                if p.is_destination(x, y) {
                    self.path = None;
                }
                true
            }
            None => false,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, x1: u32, y1: u32, flag: &Image) -> GameResult {
        let dx = self.x as f64 - x1 as f64;
        let dy = self.y as f64 - y1 as f64;

        // self.batch.add(
        //     DrawParam::new()
        //         .dest([dx as f32, dy as f32])
        //         .offset([0.5, 0.5]),
        // );

        // tank
        graphics::draw(
            ctx,
            &self.cfg.image,
            DrawParam::new()
                .dest([dx as f32, dy as f32])
                .offset([0.5, 0.5])
                .rotation(self.angle),
        )?;

        // barrel
        graphics::draw(
            ctx,
            &self.cfg.barrel,
            DrawParam::new()
                .dest([dx as f32, dy as f32])
                .offset([0.5, 0.1])
                .rotation(self.barrel_angle),
        )?;

        //        crate::util::debug::draw_circle(ctx, dx as f32, dy as f32, 1.0)?;

        // flag
        if let Some(p) = &self.path {
            let fx = p.x2 as f64 - x1 as f64;
            let fy = p.y2 as f64 - y1 as f64;

            graphics::draw(
                ctx,
                flag,
                DrawParam::new()
                    .dest([fx as f32, fy as f32])
                    .offset([0.5, 0.5]),
            )?;
        }

        Ok(())
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }
}
