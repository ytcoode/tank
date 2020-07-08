use ggez::event::EventHandler;
use ggez::event::MouseButton;
use ggez::graphics;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::input::keyboard::KeyCode;
use ggez::input::keyboard::KeyMods;
use ggez::timer;
use ggez::Context;
use ggez::GameResult;
use std::rc::Rc;
use std::time::Instant;

mod cfg;
use cfg::*;

mod map;
use map::*;

mod path;

mod tank;
use tank::bullet::Bullet;
use tank::Tank;

mod vision;
use vision::*;

pub struct Game {
    cfgs: GameCfgs,
    map: Map,

    tanks: Vec<Rc<Tank>>,
    tank: Rc<Tank>, // player-controlled tank
    tank_sprites: Vec<SpriteBatch>,

    bullets: Vec<Bullet>,
    vision: Vision,
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Game> {
        let cfgs = GameCfgs::load(ctx);
        let map = Map::new(ctx)?;

        let tanks = Vec::new();
        let tank_cfg = cfgs.tanks.get(0).expect("Tank{id: 0} not found!");
        let tank = Tank::new(tank_cfg.clone(), 100, 100);
        let tank_sprites = cfgs
            .tanks
            .cfgs
            .iter()
            .map(|c| SpriteBatch::new(c.image.clone()))
            .collect();

        let bullets = Vec::new();
        let vision = Vision::new(tank.x(), tank.y(), &map, ctx);

        Ok(Game {
            cfgs,
            map,

            tanks,
            tank,
            tank_sprites,

            bullets,
            vision,
        })
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let now = Instant::now();

        for i in (0..self.tanks.len()).rev() {
            let tank = self.tanks[i];
            if tank.destroyed() {
            } else {
                self.tanks[i].update(self, now);
            }
        }

        if self.tank.update(now) {
            self.vision.update(self.tank.x(), self.tank.y(), &self.map);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // clear
        graphics::clear(ctx, graphics::WHITE);

        // title
        graphics::set_window_title(ctx, &format!("Tanks - {:.0} FPS", timer::fps(ctx),));

        // vision
        let vision = &self.vision;

        // map
        self.map
            .draw(ctx, vision.x1, vision.x2, vision.y1, vision.y2)?;

        // tank
        self.tank
            .draw(ctx, vision.x1, vision.y1, &self.images.flag)?;

        // debug
        //        debug::draw_axis(ctx)?;
        //debug::draw_circle(ctx, self.tank.dx - x1 as f32, self.tank.dy - y1 as f32, 5.0)?;

        // present
        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Key1 => self.tank.barrel_rotation = -0.03,
            KeyCode::Key2 => self.tank.barrel_rotation = 0.03,
            KeyCode::Space => self.tank.fire(Instant::now()),
            KeyCode::Escape => ggez::event::quit(ctx),
            _ => (),
        }
    }

    /// A keyboard button was released.
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        match keycode {
            KeyCode::Key1 => self.tank.barrel_rotation = 0.0,
            KeyCode::Key2 => self.tank.barrel_rotation = 0.0,
            _ => (),
        }
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) {
        let x = self.vision.x1 + x as u32;
        let y = self.vision.y1 + y as u32;
        self.tank.move_to(x, y, Instant::now());
    }
}
