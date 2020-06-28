use ggez::graphics;
use ggez::Context;
use ggez::GameResult;

pub fn draw_line(ctx: &mut Context, x1: f32, y1: f32, x2: f32, y2: f32) -> GameResult {
    let line = graphics::Mesh::new_line(ctx, &[[x1, y1], [x2, y2]], 1.0, graphics::BLACK)?;
    graphics::draw(ctx, &line, ([0.0, 0.0],))
}

pub fn velocity(vx: f32, vy: f32, vt: f32) -> (f32, f32) {
    if vx == 0.0 {
        return if vy > 0.0 { (0.0, vt) } else { (0.0, -vt) };
    }
    if vy == 0.0 {
        return if vx > 0.0 { (vt, 0.0) } else { (-vt, 0.0) };
    }

    let mut x = vt.abs() * std::f32::consts::FRAC_PI_4.cos();
    let mut y = x;

    if vx < 0.0 {
        x = -x;
    }

    if vy < 0.0 {
        y = -y
    }

    (x, y)
}
