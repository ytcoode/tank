use super::tank;
use ggez::graphics;
use ggez::graphics::DrawParam;
use ggez::graphics::Image;
use ggez::Context;
use ggez::GameResult;
use std::rc::Rc;
use std::time::Instant;

pub BulletCfg {
    image: Image,
    speed: u16,
}

impl BulletCfg{
pub    fn new<C: Config>(c: &C, tank_id: u16, ctx: &mut Context) -> BulletCfg {
	        let image = c
            .str("bullet")
            .not_empty()
            .map(|s| {
                Image::new(ctx, s.get())
                    .expect(format!("TankCfg{{id: {}}} bullet not found", tank_id).as_str())
            })
            .get();

	let speed = c.u16("bullet_speed").ge(1).get();

	BulletCfg {
	    image, speed,
	}

    }
}

pub struct Bullet {
    tank: Rc<Tank>,
    x: u32,
    y: u32,
    angle: f32,
    time: Instant, // start time
}

impl Bullet {
    pub fn new(tank: Rc<Tank>, x: u32, y: u32, angle: f32, time: Instant) -> Bullet {
        Bullet {
            tank,
            x,
            y,
            angle,
            time,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, vx: u32, vy: u32, now: Instant) -> GameResult {
        let dt = now.duration_since(self.time).as_secs_f64();
        let distance = dt * 200.0;

        let dx = self.angle.cos() * distance;
        let dy = self.angle.sin() * distance;

        self.cx = (self.sx as f64 + dx) as u32;
        self.cy = (self.sy as f64 + dy) as u32;

        let dx = self.cx as f64 - x1 as f64;
        let dy = self.cy as f64 - y1 as f64;

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
