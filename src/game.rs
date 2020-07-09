use cfg::GameCfgs;
use ggez::event::EventHandler;
use ggez::event::MouseButton;
use ggez::graphics;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::input::keyboard::KeyCode;
use ggez::input::keyboard::KeyMods;
use ggez::timer;
use ggez::Context;
use ggez::GameResult;
use map::Map;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;
use tank::bullet::Bullet;
use tank::Tank;
use vision::Vision;

mod cfg;
mod map;
mod path;
mod tank;
mod update;
mod vision;

pub struct Game {
    cfgs: GameCfgs,
    map: Map,

    tanks: Vec<Rc<RefCell<Tank>>>,
    tank: Rc<RefCell<Tank>>, // player-controlled tank

    bullets: Vec<Bullet>,
    vision: Vision,
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Game> {
        let cfgs = GameCfgs::load(ctx);
        let map = Map::new(ctx)?;

        let tanks = Vec::new();
        let tank_cfg = cfgs.tanks.get(0).expect("Tank{id: 0} not found!");
        let tank = Rc::new(RefCell::new(Tank::new(tank_cfg.clone(), 100, 100)));

        let bullets = Vec::new();
        let vision = Vision::new(tank.borrow().x(), tank.borrow().y(), &map, ctx);

        Ok(Game {
            cfgs,
            map,

            tanks,
            tank,

            bullets,
            vision,
        })
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let now = Instant::now();

        for i in (0..self.tanks.len()).rev() {
            let e = &self.tanks[i];
            if e.borrow().destroyed() {
                self.tanks.swap_remove(i);
            } else {
                e.borrow_mut().update(self, now);
            }
        }

        // let x = self.tank.borrow().x();
        // let y = self.tank.borrow().y();

        // update::update(&mut self.tanks, self, now);

        // if self.tank.update(self, now) {
        //     self.vision.update(self.tank.x(), self.tank.y(), &self.map);
        // }

        // self.tank.borrow_mut().update(self, now);
        // self.tank.borrow_mut().update(self, now);

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
            .borrow()
            .draw(ctx, vision.x1, vision.y1, &self.cfgs.misc.flag)?;

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
            // KeyCode::Key1 => self.tank.barrel_rotation = -0.03,
            // KeyCode::Key2 => self.tank.barrel_rotation = 0.03,
            // KeyCode::Space => self.tank.fire(Instant::now()),
            KeyCode::Escape => ggez::event::quit(ctx),
            _ => (),
        }
    }

    /// A keyboard button was released.
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        match keycode {
            // KeyCode::Key1 => self.tank.barrel_rotation = 0.0,
            // KeyCode::Key2 => self.tank.barrel_rotation = 0.0,
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
        // let x = self.vision.x1 + x as u32;
        // let y = self.vision.y1 + y as u32;
        // self.tank.move_to(x, y, Instant::now());
    }
}
