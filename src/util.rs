pub const POINT_ZERO: [f32; 2] = [0.0, 0.0];
pub const DRAW_PARAM_ZERO: ([f32; 2],) = (POINT_ZERO,);

pub fn velocity(vx: f32, vy: f32, vt: f32) -> (f32, f32) {
    if vx == 0.0 && vy == 0.0 {
        return (0.0, 0.0);
    }

    if vx == 0.0 {
        return if vy > 0.0 { (0.0, vt) } else { (0.0, -vt) };
    }

    if vy == 0.0 {
        return if vx > 0.0 { (vt, 0.0) } else { (-vt, 0.0) };
    }

    let mut x = vt * std::f32::consts::FRAC_PI_4.cos();
    let mut y = x;

    if vx < 0.0 {
        x = -x;
    }

    if vy < 0.0 {
        y = -y
    }

    (x, y)
}

pub fn distance(x1: u32, y1: u32, x2: u32, y2: u32) -> f64 {
    let dx = if x1 < x2 { x2 - x1 } else { x1 - x2 } as f64;
    let dy = if y1 < y2 { y2 - y1 } else { y1 - y2 } as f64;
    dx.hypot(dy)
}
