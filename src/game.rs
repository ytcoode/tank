use crate::image;
use crate::image::Images;
use crate::map::Map;
use crate::tank::Tank;
use ggez::event::EventHandler;
use ggez::event::MouseButton;
use ggez::graphics;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::input::keyboard::KeyCode;
use ggez::input::keyboard::KeyMods;
use ggez::timer;
use ggez::Context;
use ggez::GameResult;
use std::time::Instant;

mod cfg;
use cfg::*;

mod vision;
use vision::*;

pub struct GameState {
    _cfgs: GameCfgs,
    images: Images,
    map: Map,
    tank: Tank,
    _tank_sprites: Vec<SpriteBatch>,
    vision: Vision,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        let cfgs = cfg::load(ctx);
        let images = image::load(ctx);

        let map = Map::new(ctx)?;

        let tank_cfg = cfgs
            .tanks
            .cfgs
            .get(0)
            .expect("Tank config file cannot be empty!");

        let tank = Tank::new(tank_cfg.clone(), 100, 100);

        let tank_sprites = cfgs
            .tanks
            .cfgs
            .iter()
            .map(|c| SpriteBatch::new(c.image.clone()))
            .collect();

        let vision = Vision::new(tank.x(), tank.y(), &map, ctx);

        Ok(GameState {
            _cfgs: cfgs,
            images,
            map,
            tank,
            _tank_sprites: tank_sprites,
            vision,
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let now = Instant::now();
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
