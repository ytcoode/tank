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
