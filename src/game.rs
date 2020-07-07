use crate::map::Map;
use crate::tank::Tank;
use ggez::event::EventHandler;
use ggez::event::MouseButton;
use ggez::graphics;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::timer;
use ggez::Context;
use ggez::GameResult;
use std::time::Instant;

mod cfg;
use cfg::*;

pub struct GameState {
    _cfgs: GameCfgs,
    map: Map,
    tank: Tank,
    _tank_sprites: Vec<SpriteBatch>,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        let cfgs = cfg::load_cfgs(ctx);
        let map = Map::new(ctx)?;

        let tank_cfg = cfgs
            .tanks
            .cfgs
            .get(0)
            .expect("Tank config file cannot be empty!");

        let tank = Tank::new(tank_cfg.clone(), 0, 0);

        let tank_sprites = cfgs
            .tanks
            .cfgs
            .iter()
            .map(|c| SpriteBatch::new(c.image.clone()))
            .collect();

        Ok(GameState {
            _cfgs: cfgs,
            map,
            tank,
            _tank_sprites: tank_sprites,
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let now = Instant::now();

        self.tank.update(now);

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

        // vision
        let (x1, x2, y1, y2) = self.map.vision(ctx, self.tank.x(), self.tank.y());
        self.tank.x1 = x1;
        self.tank.y1 = y1;

        // map
        self.map.draw(ctx, x1, x2, y1, y2)?;

        // tank
        self.tank.draw(ctx, x1, y1)?;

        // debug
        //        debug::draw_axis(ctx)?;
        //debug::draw_circle(ctx, self.tank.dx - x1 as f32, self.tank.dy - y1 as f32, 5.0)?;

        // misc
        graphics::set_window_title(ctx, &format!("Tanks - {:.0} FPS", timer::fps(ctx),));

        // present
        graphics::present(ctx)
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
        let x = self.tank.x1 + x as u32;
        let y = self.tank.y1 + y as u32;

        self.tank.move_to(x, y, Instant::now());

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
