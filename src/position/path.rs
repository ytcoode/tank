use std::time::Instant;

#[derive(Debug)]
pub struct Path {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
    start_time: Instant,
}

impl Path {
    fn position(now: Instant) -> Option<(u32, u32)> {
        None
    }
}

pub fn new(x1: u32, y1: u32, x2: u32, y2: u32) -> Path {
    Path {
        x1,
        y1,
        x2,
        y2,
        start_time: Instant::now(),
    }
}
