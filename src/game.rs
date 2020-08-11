use self::cfg::GameCfgs;
use self::scene::Scene;
use self::tank::Tank;
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::timer;
use ggez::{Context, GameResult};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

mod cfg;
mod common;
mod scene;
mod tank;

pub struct Game {
    cfgs: GameCfgs,
    scene: Scene,
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Game> {
        let cfgs = GameCfgs::load(ctx);

        let scene_cfg = cfgs
            .scenes
            .get(1)
            .expect(format!("The scene config with an ID of {} not found!", 1).as_str());

        let scene = Scene::new(scene_cfg.clone(), ctx);

        Ok(Game { cfgs, scene })
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let now = Instant::now();
        self.scene.update(now);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // clear
        graphics::clear(ctx, graphics::WHITE);

        // title
        graphics::set_window_title(ctx, &format!("Tanks - {:.0} FPS", timer::fps(ctx),));

        // scene
        self.scene.draw(ctx);

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
        self.scene
            .player_tank_move_to(x as u32, y as u32, Instant::now());
    }
}
