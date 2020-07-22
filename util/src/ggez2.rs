use ggez::graphics;
use ggez::Context;

pub fn draw_line(ctx: &mut Context, x1: f32, y1: f32, x2: f32, y2: f32) {
    let line = graphics::Mesh::new_line(ctx, &[[x1, y1], [x2, y2]], 0.6, graphics::BLACK).unwrap();
    graphics::draw(ctx, &line, ([0.0, 0.0],)).unwrap()
}

// pub fn draw_axis(ctx: &mut Context) -> GameResult {
//     let (w, h) = graphics::drawable_size(ctx);

//     // x
//     //  draw_line(ctx, 0.0, 1.0, w, 1.0)?;
//     draw_line(ctx, 10.0, h / 2.0, w - 10.0, h / 2.0)?;
//     //    draw_line(ctx, 0.0, h - 1.0, w, h - 1.0)?;

//     // y
//     //    draw_line(ctx, 1.0, 0.0, 1.0, h)?;
//     draw_line(ctx, w / 2.0, 10.0, w / 2.0, h - 10.0)?;
//     //  draw_line(ctx, w - 1.0, 0.0, w - 1.0, h)?;

//     Ok(())
// }

// pub fn draw_circle(ctx: &mut Context, x1: f32, y1: f32, r: f32) -> GameResult {
//     let circle = graphics::Mesh::new_circle(
//         ctx,
//         graphics::DrawMode::stroke(1.0),
//         [x1, y1],
//         r,
//         1.0,
//         graphics::BLACK,
//     )?;
//     graphics::draw(ctx, &circle, ([0.0, 0.0],))
// }
