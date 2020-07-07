use crate::util;
use std::time::Instant;

#[derive(Debug)]
pub struct Path {
    x1: u32,
    y1: u32,
    pub x2: u32,
    pub y2: u32,

    dt: f64,     // seconds needed to reach the destination
    st: Instant, // start time
}

impl Path {
    pub fn new(x1: u32, y1: u32, x2: u32, y2: u32, speed: u16, st: Instant) -> Path {
        let dp = util::distance(x1, y1, x2, y2);
        let dt = dp / speed as f64;
        Path {
            x1,
            y1,
            x2,
            y2,
            dt,
            st,
        }
    }

    pub fn position(&self, now: Instant) -> (u32, u32) {
        let dt = now.duration_since(self.st).as_secs_f64();
        if dt >= self.dt {
            return (self.x2, self.y2);
        }

        let ratio = dt / self.dt;

        let dx = (self.x2 as f64 - self.x1 as f64) * ratio;
        let dy = (self.y2 as f64 - self.y1 as f64) * ratio;

        let cx = self.x1 as f64 + dx;
        let cy = self.y1 as f64 + dy;

        (cx as u32, cy as u32)
    }

    pub fn is_destination(&self, x: u32, y: u32) -> bool {
        return self.x2 == x && self.y2 == y;
    }
}
