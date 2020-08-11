mod ggez2;
pub use ggez2::*;
pub mod byte;
pub mod debug;
pub mod file;

pub fn div_ceil(a: u32, b: u32) -> u32 {
    (a + b - 1) / b
}

pub fn substract(a: u32, b: u32) -> f32 {
    if a > b {
        (a - b) as f32
    } else {
        -((b - a) as f32)
    }
}

pub const POINT_ZERO: [f32; 2] = [0.0, 0.0];
pub const DRAW_PARAM_ZERO: ([f32; 2],) = (POINT_ZERO,);

pub enum Ref<K, V> {
    ID(K),
    Obj(V),
}

pub fn distance(x1: u32, y1: u32, x2: u32, y2: u32) -> f64 {
    let dx = if x1 < x2 { x2 - x1 } else { x1 - x2 } as f64;
    let dy = if y1 < y2 { y2 - y1 } else { y1 - y2 } as f64;
    dx.hypot(dy)
}

pub fn push<T>(mut vec: Vec<T>, val: T) -> Vec<T> {
    vec.push(val);
    vec
}

pub fn is_inside_rectangle(i1: u32, i2: u32, j1: u32, j2: u32, a: u32, b: u32) -> bool {
    a >= i1 && a < i2 && b >= j1 && b < j2
}
