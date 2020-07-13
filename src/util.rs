pub mod byte;
pub mod debug;
pub mod file;

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
