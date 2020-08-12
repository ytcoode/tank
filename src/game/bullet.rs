use crate::game::common::view::PlayerView;
use crate::game::scene::unit::{MapCell, Unit, View};
use crate::game::tank::Tank;
use config::Config;
use ggez::graphics::{self, DrawParam, Image};
use ggez::Context;
use std::cell::Cell;
use std::fmt;
use std::rc::Rc;
use std::time::Instant;

#[derive(Debug)]
pub struct BulletCfg {
    image: Image,
    speed: u32,
}

impl BulletCfg {
    pub fn new(c: &impl Config, ctx: &mut Context) -> BulletCfg {
        let image = c
            .str("bullet_image")
            .not_empty()
            .map(|s| Image::new(ctx, s.get()).expect(format!("{} not found", s).as_str()))
            .get();

        let speed = c.u32("bullet_speed").ge(1).get();

        BulletCfg { image, speed }
    }
}

pub struct Bullet {
    id: u32,
    x0: u32,
    y0: u32,
    x: Cell<u32>,
    y: Cell<u32>,
    angle: f64,
    time: Instant, // start time
    cfg: Rc<BulletCfg>,
    tank: Rc<Tank>,
    map_cell: MapCell,
}

impl Bullet {
    pub fn new(
        id: u32,
        x: u32,
        y: u32,
        angle: f64,
        time: Instant,
        cfg: Rc<BulletCfg>,
        tank: Rc<Tank>,
    ) -> Bullet {
        Bullet {
            id,
            x0: x,
            y0: y,
            x: Cell::new(x),
            y: Cell::new(y),
            angle,
            time,
            cfg,
            tank,
            map_cell: Default::default(),
        }
    }

    pub fn update(&self, now: Instant) {
        let dt = now.duration_since(self.time).as_secs_f64();
        let dt = dt * self.cfg.speed as f64;

        let dx = self.angle.cos() * dt;
        let dy = self.angle.sin() * dt;

        let x = self.x0 as f64 + dx;
        let y = self.y0 as f64 + dy;

        let scene = &self.tank.scene;

        if x < 0.0 || y < 0.0 {
            scene.destroyed_bullets.borrow_mut().push(self.id);
            return;
        }

        let x = x as u32;
        let y = y as u32;

        let (width, height) = scene.size();

        if x >= width || y >= height {
            scene.destroyed_bullets.borrow_mut().push(self.id);
            return;
        }

        self.x.set(x as u32);
        self.y.set(y as u32);
    }
}

impl fmt::Display for Bullet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bullet({})", self.id)
    }
}

impl fmt::Debug for Bullet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bullet({})", self.id)
    }
}

impl Unit for Bullet {
    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        "bullet"
    }

    fn position(&self) -> (u32, u32) {
        (self.x.get(), self.y.get())
    }

    fn view(&self) -> Option<&View> {
        None
    }

    fn view_enter(&self, viewer: &dyn Unit) {}

    fn view_leave(&self, viewer: &dyn Unit) {}

    fn map_cell(&self) -> &MapCell {
        &self.map_cell
    }

    fn draw(&self, ctx: &mut Context, view: &PlayerView) {
        let dx = self.x.get() as f64 - view.x as f64;
        let dy = self.y.get() as f64 - view.y as f64;

        graphics::draw(
            ctx,
            &self.cfg.image,
            DrawParam::new()
                .dest([dx as f32, dy as f32])
                .offset([0.5, 0.5])
                .rotation(self.angle as f32),
        )
        .unwrap();
    }
}
