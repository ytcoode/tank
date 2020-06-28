use ggez::graphics;
use ggez::Context;
use ggez::GameResult;

pub fn draw_line(ctx: &mut Context, x1: f32, y1: f32, x2: f32, y2: f32) -> GameResult {
    let line = graphics::Mesh::new_line(ctx, &[[x1, y1], [x2, y2]], 1.0, graphics::BLACK)?;
    graphics::draw(ctx, &line, ([0.0, 0.0],))
}
