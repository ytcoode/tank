use ggez;
use ggez::event;
use ggez::event::EventHandler;
use ggez::graphics;
use ggez::nalgebra;
use ggez::Context;
use ggez::GameResult;

struct GameState {
    pos_x: f32,
}

impl GameState {
    fn new() -> GameResult<GameState> {
        let s = GameState { pos_x: 0.0 };
        Ok(s)
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::WHITE);

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            nalgebra::Point2::new(0.0, 0.0),
            100.0,
            0.1,
            graphics::BLACK,
        )?;
        graphics::draw(ctx, &circle, (nalgebra::Point2::new(self.pos_x, 380.0),))?;

        graphics::present(ctx)
    }
}

pub fn main() -> GameResult {
    let (mut ctx, mut event_loop) = ggez::ContextBuilder::new("super_simple", "ggez").build()?;
    let mut state = GameState::new()?;
    event::run(&mut ctx, &mut event_loop, &mut state)
}
