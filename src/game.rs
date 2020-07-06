use crate::debug;
use crate::map::Map;
use crate::tank::Tank;
use crate::util;
use ggez::event;
use ggez::event::EventHandler;
use ggez::event::MouseButton;
use ggez::graphics;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::input::keyboard::KeyCode;
use ggez::input::keyboard::KeyMods;
use ggez::timer;
use ggez::Context;
use ggez::GameResult;

mod cfg;
use cfg::*;

pub struct GameState {
    cfgs: GameCfgs,
    map: Map,
    tank: Tank,
    tank_sprites: Vec<SpriteBatch>,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        let cfgs = cfg::load_cfgs(ctx);
        let map = Map::new(ctx)?;

        let tankCfg = cfgs
            .tanks
            .cfgs
            .get(0)
            .expect("Tank config file cannot be empty!");

        let tank = Tank::new(tankCfg.clone());

        let tank_sprites = cfgs
            .tanks
            .cfgs
            .iter()
            .map(|c| SpriteBatch::new(c.image.clone()))
            .collect();

        Ok(GameState {
            cfgs,
            map,
            tank,
            tank_sprites,
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // let t = &mut self.tank;
        // t.x += t.vx;
        // t.y += t.vy;
        // // t.x =
        // //     t.x.max(t.width as f32 / 2.0)
        // //         .min(self.map.width as f32 - t.width as f32 / 2.0);
        // // t.y =
        // //     t.y.max(t.height as f32 / 2.0)
        // //         .min(self.map.height as f32 - t.height as f32 / 2.0);

        // if (t.x - t.dx).abs() <= 4.0 && (t.y - t.dy).abs() <= 4.0 {
        //     t.vx = 0.0;
        //     t.vy = 0.0;
        // }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // clear
        graphics::clear(ctx, graphics::WHITE);

        // // vision
        // let (x1, x2, y1, y2) = self.map.vision(ctx, self.tank.x as u32, self.tank.y as u32);
        // self.tank.x1 = x1 as f32;
        // self.tank.y1 = y1 as f32;

        // // map
        // self.map.draw(ctx, x1, x2, y1, y2)?;

        // // tank
        // self.tank.draw(ctx, x1, y1)?;

        // // debug
        // debug::draw_axis(ctx)?;
        // debug::draw_circle(ctx, self.tank.dx - x1 as f32, self.tank.dy - y1 as f32, 5.0)?;

        // misc
        graphics::set_window_title(ctx, &format!("Tanks - {:.0} FPS", timer::fps(ctx),));

        // present
        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        // let t = &mut self.tank;
        // match keycode {
        //     KeyCode::Up => t.vy = -4.0,
        //     KeyCode::Down => t.vy = 4.0,
        //     KeyCode::Left => t.vx = -4.0,
        //     KeyCode::Right => t.vx = 4.0,
        //     _ => (),
        // }

        // let (vx, vy) = util::velocity(t.vx, t.vy, 4.0);
        // t.vx = vx;
        // t.vy = vy;
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        // let t = &mut self.tank;
        // match keycode {
        //     KeyCode::Up => t.vy = 0.0,
        //     KeyCode::Down => t.vy = 0.0,
        //     KeyCode::Left => t.vx = 0.0,
        //     KeyCode::Right => t.vx = 0.0,
        //     KeyCode::Escape => event::quit(ctx),
        //     _ => (),
        // }
        // let (vx, vy) = util::velocity(t.vx, t.vy, 4.0);
        // t.vx = vx;
        // t.vy = vy;
    }

    /// A mouse button was pressed
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
    }

    /// A mouse button was released
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: f32, y: f32) {
        // let x = x + self.tank.x1;
        // let y = y + self.tank.y1;

        // self.tank.dx = x;
        // self.tank.dy = y;
        // self.tank.x0 = self.tank.x;
        // self.tank.y0 = self.tank.y;

        // let angle = (y - self.tank.y).atan2(x - self.tank.x);
        // self.tank.vx = angle.cos() * 4.0;
        // self.tank.vy = angle.sin() * 4.0;
    }
}
