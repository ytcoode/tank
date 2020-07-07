use path::Path;
use std::time::Instant;

mod path;

pub struct Position {
    x: u32,
    y: u32,
    path: Option<Path>,
}

impl Position {
    pub fn new(x: u32, y: u32) -> Position {
        Position { x, y, path: None }
    }

    pub fn move_to(&mut self, x: u32, y: u32, speed: u16, now: Instant) {
        self.path = Some(Path::new(self.x, self.y, x, y, speed, now));
    }

    pub fn update(&mut self, now: Instant) -> bool {
        match self.path {
            Some(ref p) => {
                let (x, y) = p.position(now);
                self.x = x;
                self.y = y;
                if p.is_destination(x, y) {
                    self.path = None;
                }
                true
            }
            None => false,
        }
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }
}
