use ggez;
use ggez::event;
use ggez::event::EventHandler;
use ggez::graphics;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::Image;
use ggez::input::keyboard::KeyCode;
use ggez::input::keyboard::KeyMods;
use ggez::nalgebra::Point2;
use ggez::timer;
use ggez::Context;
use ggez::ContextBuilder;
use ggez::GameResult;
use map::Map;
use tank::Tank;

mod debug;
mod map;
mod tank;
mod util;
mod vision;

struct GameState {
    tanks: Vec<Tank>,
    tank_batch: SpriteBatch,
    map: Map,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let mut tanks = Vec::new();
        tanks.push(Tank::new());

        let tank_texture = Image::new(ctx, "/PNG/Tanks/tankGreen.png")?;
        let tank_batch = SpriteBatch::new(tank_texture);

        let map = Map::new(ctx)?;

        Ok(GameState {
            tanks,
            tank_batch,
            map,
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.tanks.iter_mut().for_each(|t| {
            t.x += t.vx;
            t.y += t.vy;
            t.x = t.x.max(0.0).min(map::MAP_WIDTH - 1.0);
            t.y = t.y.max(0.0).min(map::MAP_HEIGHT - 1.0);
        });
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // clear
        graphics::clear(ctx, graphics::WHITE);

        // map
        let tank = &self.tanks[0];
        let (x1, y1) = self.map.draw(ctx, tank.x, tank.y)?;

        // debug
        debug::draw_axis(ctx)?;

        // tank
        for tank in &self.tanks {
            self.tank_batch
                .add((Point2::new(tank.x - x1 - 37.5, tank.y - y1 - 35.0),));
        }
        graphics::draw(ctx, &self.tank_batch, util::DRAW_PARAM_ZERO)?;
        self.tank_batch.clear();

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
        let tank = &mut self.tanks[0];
        match keycode {
            KeyCode::Up => tank.vy = -4.0,
            KeyCode::Down => tank.vy = 4.0,
            KeyCode::Left => tank.vx = -4.0,
            KeyCode::Right => tank.vx = 4.0,
            _ => (),
        }

        let (vx, vy) = util::velocity(tank.vx, tank.vy, 4.0);
        tank.vx = vx;
        tank.vy = vy;
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        let tank = &mut self.tanks[0];
        match keycode {
            KeyCode::Up => tank.vy = 0.0,
            KeyCode::Down => tank.vy = 0.0,
            KeyCode::Left => tank.vx = 0.0,
            KeyCode::Right => tank.vx = 0.0,
            KeyCode::Escape => event::quit(ctx),
            _ => (),
        }
        let (vx, vy) = util::velocity(tank.vx, tank.vy, 4.0);
        tank.vx = vx;
        tank.vy = vy;
    }
}

pub fn main() -> GameResult {
    let (mut ctx, mut event_loop) = ContextBuilder::new("tank", "bunny")
        .add_resource_path("./resources")
        .build()?;

    let mut state = GameState::new(&mut ctx)?;

    event::run(&mut ctx, &mut event_loop, &mut state)
}
