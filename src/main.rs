mod map;

use ggez;
use ggez::event;
use ggez::event::EventHandler;
use ggez::graphics;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::DrawParam;
use ggez::graphics::Image;
use ggez::input::keyboard::KeyCode;
use ggez::input::keyboard::KeyMods;
use ggez::nalgebra::Point2;
use ggez::nalgebra::Vector2;
use ggez::timer;
use ggez::Context;
use ggez::ContextBuilder;
use ggez::GameResult;
use map::Map;

struct Tank {
    position: Point2<f32>,
    velocity: Vector2<f32>,
}

impl Tank {
    fn new() -> Tank {
        Tank {
            position: Point2::new(0.0, 0.0),
            velocity: Vector2::new(0.0, 0.0),
        }
    }
}

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
            t.position += t.velocity;
            t.velocity.x = 0.0;
            t.velocity.y = 0.0;
        });
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // clear
        graphics::clear(ctx, graphics::WHITE);

        // tile
        self.map.draw(ctx)?;

        // tank
        for tank in &self.tanks {
            self.tank_batch.add((tank.position,));
        }
        graphics::draw(
            ctx,
            &self.tank_batch,
            DrawParam::new()
                .dest(Point2::new(0.0, 0.0))
                .scale(Vector2::new(0.5, 0.5)),
        )?;
        self.tank_batch.clear();

        // misc
        graphics::set_window_title(ctx, &format!("Tanks - {:.0} FPS", timer::fps(ctx),));

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
        let tank = &mut self.tanks[0];
        match keycode {
            KeyCode::Up => tank.velocity.y = -15.0,
            KeyCode::Down => tank.velocity.y = 15.0,
            KeyCode::Left => tank.velocity.x = -15.0,
            KeyCode::Right => tank.velocity.x = 15.0,
            KeyCode::Escape => event::quit(ctx),
            _ => (),
        }
    }
}

pub fn main() -> GameResult {
    let (mut ctx, mut event_loop) = ContextBuilder::new("tank", "bunny")
        .add_resource_path("./resources")
        .build()?;

    let mut state = GameState::new(&mut ctx)?;

    event::run(&mut ctx, &mut event_loop, &mut state)
}
