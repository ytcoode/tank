use crate::debug;
use crate::map::Map;
use crate::tank::Tank;
use crate::util;
use ggez;
use ggez::event;
use ggez::event::EventHandler;
use ggez::graphics;
use ggez::input::keyboard::KeyCode;
use ggez::input::keyboard::KeyMods;
use ggez::timer;
use ggez::Context;
use ggez::GameResult;

pub struct GameState {
    map: Map,
    tank: Tank,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        let map = Map::new(ctx)?;
        let tank = Tank::new(ctx)?;

        Ok(GameState { map, tank })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let t = &mut self.tank;
        t.x += t.vx;
        t.y += t.vy;
        t.x = t.x.max(0.0).min((self.map.width - t.width) as f32);
        t.y = t.y.max(0.0).min((self.map.height - t.height) as f32);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // clear
        graphics::clear(ctx, graphics::WHITE);

        // vision
        let (x1, x2, y1, y2) = self.map.vision(ctx, self.tank.x as u32, self.tank.y as u32);

        // map
        self.map.draw(ctx, x1, x2, y1, y2)?;

        // debug
        debug::draw_axis(ctx)?;

        // tank
        self.tank.draw(ctx, x1, y1)?;

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
        let t = &mut self.tank;
        match keycode {
            KeyCode::Up => t.vy = -4.0,
            KeyCode::Down => t.vy = 4.0,
            KeyCode::Left => t.vx = -4.0,
            KeyCode::Right => t.vx = 4.0,
            _ => (),
        }

        let (vx, vy) = util::velocity(t.vx, t.vy, 4.0);
        t.vx = vx;
        t.vy = vy;
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        let t = &mut self.tank;
        match keycode {
            KeyCode::Up => t.vy = 0.0,
            KeyCode::Down => t.vy = 0.0,
            KeyCode::Left => t.vx = 0.0,
            KeyCode::Right => t.vx = 0.0,
            KeyCode::Escape => event::quit(ctx),
            _ => (),
        }
        let (vx, vy) = util::velocity(t.vx, t.vy, 4.0);
        t.vx = vx;
        t.vy = vy;
    }
}
