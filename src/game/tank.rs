use crate::game::path::Path;
use crate::game::update::Update;
use crate::game::Game;
use config::{self, Config};
use ggez::graphics::{self, DrawParam, Image};
use ggez::{Context, GameResult};
use std::convert::{TryFrom, TryInto};
use std::f64;
use std::rc::Rc;
use std::time::Instant;

pub mod bullet;
use bullet::*;

#[derive(Debug)]
pub struct TankCfgs {
    pub cfgs: Vec<Rc<TankCfg>>,
}

impl TankCfgs {
    pub fn load(ctx: &mut Context) -> TankCfgs {
        let cfgs = config::load("config/tank.txt")
            .into_iter()
            .map(|c| TankCfg::new(c, ctx))
            .enumerate()
            .map(|(i, t)| {
                assert_eq!(
                    usize::try_from(t.id).unwrap(),
                    i,
                    "Tank id must start at zero and increase sequentially!"
                );
                t
            })
            .collect();

        TankCfgs { cfgs }
    }

    pub fn get(&self, id: u16) -> Option<&Rc<TankCfg>> {
        self.cfgs.get(usize::try_from(id).unwrap())
    }
}

#[derive(Debug)]
pub struct TankCfg {
    id: u16,
    image: Image,
    image_barrel: Image,
    speed: u16,
    bullet: BulletCfg,
}

impl TankCfg {
    fn new(c: impl Config, ctx: &mut Context) -> Rc<TankCfg> {
        let id = c.u16("id").get();

        let image = c
            .str("image")
            .not_empty()
            .map(|s| Image::new(ctx, s.get()).expect(format!("{} not found", s).as_str()))
            .get();

        let image_barrel = c
            .str("image_barrel")
            .not_empty()
            .map(|s| Image::new(ctx, s.get()).expect(format!("{} not found", s).as_str()))
            .get();

        let speed = c.u16("speed").ge(1).get();

        let bullet = BulletCfg::new(&c, ctx);

        Rc::new(TankCfg {
            id,
            image,
            image_barrel,
            speed,
            bullet,
        })
    }
}

pub struct Tank {
    cfg: Rc<TankCfg>,
    x: u32,
    y: u32,
    path: Option<Path>,
    angle: f32,
    destroyed: bool,
}

impl Tank {
    pub fn new(cfg: Rc<TankCfg>, x: u32, y: u32) -> Tank {
        Tank {
            cfg,
            x,
            y,
            path: None,
            angle: 0.0,
            destroyed: false,
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
            &self.cfg.image_barrel,
            DrawParam::new()
                .dest([dx as f32, dy as f32])
                .offset([0.5, 0.1])
                .rotation(self.angle),
        )?;

        // bullet
        // if let Some(ref mut b) = self.bullet {
        //     b.draw(ctx, x1, y1, &self.cfg.bullet)?;
        // }

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

    pub fn destroyed(&self) -> bool {
        self.destroyed
    }
}

impl Update for Tank {
    fn update(&mut self, game: &Game, now: Instant) {}

    fn destroyed(&self) -> bool {
        false
    }
}
