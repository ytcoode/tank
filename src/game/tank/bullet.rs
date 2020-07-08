use super::tank;
use ggez::graphics;
use ggez::graphics::DrawParam;
use ggez::graphics::Image;
use ggez::Context;
use ggez::GameResult;
use std::rc::Rc;
use std::time::Instant;

pub struct BulletCfg {
    image: Image,
    speed: u16,
}

impl BulletCfg {
    pub fn new<C: Config>(c: &C, ctx: &mut Context) -> BulletCfg {
        let image = c
            .str("bullet_image")
            .not_empty()
            .map(|s| Image::new(ctx, s.get()).expect(format!("{} not found", s).as_str()))
            .get();

        let speed = c.u16("bullet_speed").ge(1).get();

        BulletCfg { image, speed }
    }
}

pub struct Bullet {
    tank: Rc<Tank>,
    x: u32,
    y: u32,
    angle: f64,
    time: Instant, // start time
}

impl Bullet {
    pub fn new(tank: Rc<Tank>, x: u32, y: u32, angle: f64, time: Instant) -> Bullet {
        Bullet {
            tank,
            x,
            y,
            angle,
            time,
        }
    }

    pub fn draw(&self, ctx: &mut Context, vx: u32, vy: u32, now: Instant) -> GameResult {
        let dt = now.duration_since(self.time).as_secs_f64();
        let dt = dt * self.tank.cfg.bullet.speed as f64;

        let dx = self.angle.cos() * dt;
        let dy = self.angle.sin() * dt;

        let cx = self.x as f64 + dx;
        let cy = self.y as f64 + dy;

        let dx = cx - x1 as f64;
        let dy = cy - y1 as f64;

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
