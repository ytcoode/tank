use ggez::graphics;
use ggez::Context;
use ggez::GameResult;

use crate::util;

pub fn draw_axis(ctx: &mut Context) -> GameResult {
    let (w, h) = graphics::drawable_size(ctx);
    util::draw_line(ctx, 10.0, h / 2.0, w - 10.0, h / 2.0)?;
    util::draw_line(ctx, w / 2.0, 10.0, w / 2.0, h - 10.0)
}
