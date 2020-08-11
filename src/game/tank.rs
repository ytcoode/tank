use crate::game::common::{path::Path, view::PlayerView};
use crate::game::scene::unit::{MapCell, Unit, View};
use config::{self, Config};
use ggez::graphics::{self, DrawParam, Image};
use ggez::{Context, GameResult};
use std::cell::{Cell, RefCell};
use std::convert::{TryFrom, TryInto};
use std::f64;
use std::fmt;
use std::ops::DerefMut;
use std::rc::Rc;
use std::time::Instant;

mod cfg;
pub use cfg::*;

#[derive(Debug)]
pub struct Tank {
    id: u32,
    cfg: Rc<TankCfg>,
    x: u32,
    y: u32,
    path: Option<Path>,
    angle: f32,
    destroyed: bool,
    view: View,
    map_cell: MapCell,
}

impl fmt::Display for Tank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "tank({})", self.id)
    }
}

impl Unit for Tank {
    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        "crazy tank"
    }

    fn x(&self) -> u32 {
        self.x
    }

    fn y(&self) -> u32 {
        self.y
    }

    fn view(&self) -> Option<&View> {
        Some(&self.view)
    }

    fn view_enter(&self, viewer: &dyn Unit) {
        println!("{} came into {}'s view", self, viewer);
    }

    fn view_leave(&self, viewer: &dyn Unit) {
        println!("{} disapear from {}'s view", self, viewer);
    }

    fn map_cell(&self) -> &MapCell {
        &self.map_cell
    }

    fn draw(&self, ctx: &mut Context, view: &PlayerView) {
        let dx = self.x as f64 - view.x as f64;
        let dy = self.y as f64 - view.y as f64;

        // tank
        graphics::draw(
            ctx,
            &self.cfg.image,
            DrawParam::new()
                .dest([dx as f32, dy as f32])
                .offset([0.5, 0.5])
                .rotation(self.angle),
        )
        .unwrap();

        // barrel
        graphics::draw(
            ctx,
            &self.cfg.barrel_image,
            DrawParam::new()
                .dest([dx as f32, dy as f32])
                .offset([0.5, 0.1])
                .rotation(self.angle),
        )
        .unwrap();
    }
}

impl Tank {
    pub fn new(id: u32, cfg: Rc<TankCfg>, x: u32, y: u32) -> Tank {
        Tank {
            id,
            cfg,
            x,
            y,
            path: None,
            angle: 0.0,
            destroyed: false,
            view: View::new(100),
            map_cell: Default::default(),
        }
    }

    pub fn move_to(&mut self, x: u32, y: u32, now: Instant) {
        let path = Path::new(self.x, self.y, x, y, self.cfg.speed, now);
        let angle = path.angle() + f64::consts::FRAC_PI_2;
        self.path = Some(path);
        self.angle = angle as f32;
    }

    pub fn fire(&mut self, now: Instant) {
        //        self.bullet = Some(Bullet::new(self.x, self.y, self.barrel_angle as f64, now));
    }

    pub fn update(&mut self, now: Instant) {
        // if let Some(ref mut b) = self.bullet {
        //     b.update(now);
        // }

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
        };
    }

    pub fn draw(&self, ctx: &mut Context, x1: u32, y1: u32, flag: &Image) -> GameResult {
        let dx = self.x as f64 - x1 as f64;
        let dy = self.y as f64 - y1 as f64;

        // self.batch.add(
        //     DrawParam::new()
        //         .dest([dx as f32, dy as f32])
        //         .offset([0.5, 0.5]),
        // );

        // // tank
        // graphics::draw(
        //     ctx,
        //     &self.cfg.image,
        //     DrawParam::new()
        //         .dest([dx as f32, dy as f32])
        //         .offset([0.5, 0.5])
        //         .rotation(self.angle),
        // )?;

        // // barrel
        // graphics::draw(
        //     ctx,
        //     &self.cfg.image_barrel,
        //     DrawParam::new()
        //         .dest([dx as f32, dy as f32])
        //         .offset([0.5, 0.1])
        //         .rotation(self.angle),
        // )?;

        // bullet
        // if let Some(ref mut b) = self.bullet {
        //     b.draw(ctx, x1, y1, &self.cfg.bullet)?;
        // }

        //        crate::util::debug::draw_circle(ctx, dx as f32, dy as f32, 1.0)?;

        // // flag
        // if let Some(p) = &self.path {
        //     let fx = p.x2 as f64 - x1 as f64;
        //     let fy = p.y2 as f64 - y1 as f64;

        //     graphics::draw(
        //         ctx,
        //         flag,
        //         DrawParam::new()
        //             .dest([fx as f32, fy as f32])
        //             .offset([0.5, 0.5]),
        //     )?;
        // }

        Ok(())
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }

    pub fn destroyed(&self) -> bool {
        self.destroyed
    }
}
