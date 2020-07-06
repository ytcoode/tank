use std::time::Instant;

mod path;

pub struct Position {
    x: u32,
    y: u32,
    path: Option<path::Path>,
}

impl Position {
    pub fn new(x: u32, y: u32) -> Position {
        Position { x, y, path: None }
    }

    pub fn update(&mut self, now: Instant) {}

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }
}
