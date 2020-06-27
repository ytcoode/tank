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
use std::time;

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
    tile_width: f32,
    tile_height: f32,
    tile_batch: SpriteBatch,

    tanks: Vec<Tank>,
    tank_batch: SpriteBatch,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let tile_texture = Image::new(ctx, "/PNG/Environment/sand.png")?;
        let tile_width = tile_texture.width() as f32;
        let tile_height = tile_texture.height() as f32;
        let tile_batch = SpriteBatch::new(tile_texture);

        let mut tanks = Vec::new();
        tanks.push(Tank::new());

        let tank_texture = Image::new(ctx, "/PNG/Tanks/tankGreen.png")?;
        let tank_batch = SpriteBatch::new(tank_texture);

        Ok(GameState {
            tile_width,
            tile_height,
            tile_batch,
            tanks,
            tank_batch,
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
        let (w, h) = graphics::drawable_size(ctx);
        let mut x = 0.0;
        while x < w {
            let mut y = 0.0;
            while y < h {
                self.tile_batch.add((Point2::new(x, y),));
                y += self.tile_height;
            }
            x += self.tile_width;
        }
        graphics::draw(ctx, &self.tile_batch, (Point2::new(0.0, 0.0),))?;
        self.tile_batch.clear();

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
