use crate::game::update::Update;
use crate::game::Game;
use ggez::graphics;
use ggez::graphics::DrawParam;
use ggez::graphics::Image;
use ggez::Context;
use ggez::GameResult;
use std::rc::Rc;
use std::time::Instant;

use super::Tank;
use crate::deps::config::Config;

pub struct Bullet {
    tank: Rc<Tank>,
    x: u32,
    y: u32,
    angle: f64,
    time: Instant, // start time
    destroyed: bool,
}

impl Bullet {
    pub fn new(tank: Rc<Tank>, x: u32, y: u32, angle: f64, time: Instant) -> Bullet {
        Bullet {
            tank,
            x,
            y,
            angle,
            time,
            destroyed: false,
        }
    }

    pub fn draw(&self, ctx: &mut Context, vx: u32, vy: u32, now: Instant) -> GameResult {
        let dt = now.duration_since(self.time).as_secs_f64();
        let dt = dt * self.tank.cfg.bullet.speed as f64;

        let dx = self.angle.cos() * dt;
        let dy = self.angle.sin() * dt;

        let cx = self.x as f64 + dx;
        let cy = self.y as f64 + dy;

        let dx = cx - vx as f64;
        let dy = cy - vy as f64;

        graphics::draw(
            ctx,
            &self.tank.cfg.bullet.image,
            DrawParam::new()
                .dest([dx as f32, dy as f32])
                .offset([0.5, 0.5])
                .rotation(self.angle as f32),
        )?;

        Ok(())
    }
}

impl Update for Bullet {
    fn update(&mut self, game: &Game, now: Instant) {}

    fn destroyed(&self) -> bool {
        false
    }
}
