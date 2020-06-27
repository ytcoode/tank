use ggez;
use ggez::event;
use ggez::event::EventHandler;
use ggez::graphics;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::Color;
use ggez::nalgebra::Point2;
use ggez::Context;
use ggez::ContextBuilder;
use ggez::GameResult;

struct Tank {
    position: Point2<f32>,
}

impl Tank {
    fn new() -> Tank {
        Tank {
            position: Point2::new(0.0, 0.0),
        }
    }
}

struct GameState {
    tanks: Vec<Tank>,
    tank_batch: SpriteBatch,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let mut tanks = Vec::new();
        let tank_texture = graphics::Image::new(ctx, "/PNG/Tanks/tankGreen.png")?;
        let tank_batch = SpriteBatch::new(tank_texture);

        tanks.push(Tank::new());

        Ok(GameState { tanks, tank_batch })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from((0.392, 0.584, 0.929)));

        self.tank_batch.clear();
        for tank in &self.tanks {
            self.tank_batch.add((tank.position,));
        }
        graphics::draw(ctx, &self.tank_batch, (Point2::new(0.0, 0.0),))?;

        graphics::present(ctx)
    }
}

pub fn main() -> GameResult {
    let (mut ctx, mut event_loop) = ContextBuilder::new("tank", "bunny")
        .add_resource_path("./resources")
        .build()?;

    let mut state = GameState::new(&mut ctx)?;

    event::run(&mut ctx, &mut event_loop, &mut state)
}
