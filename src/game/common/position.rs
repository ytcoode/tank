use super::path::Path;
use std::time::Instant;

#[derive(Debug)]
pub struct Position {
    x: u32,
    y: u32,
    angle: f32,
    path: Option<Path>,
}

impl Position {
    pub fn new(x: u32, y: u32, angle: f32) -> Self {
        Self {
            x,
            y,
            angle,
            path: None,
        }
    }

    pub fn move_to(&mut self, x: u32, y: u32, speed: u32, now: Instant) {
        let path = Path::new(self.x, self.y, x, y, speed, now);
        let angle = path.angle();
        self.path = Some(path);
        self.angle = angle as f32;
    }

    pub fn update(&mut self, now: Instant) {
        match self.path {
            Some(ref p) => {
                let (x, y) = p.position(now);
                self.x = x;
                self.y = y;
                if p.is_destination(x, y) {
                    self.path = None;
                }
            }
            None => (),
        }
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }

    pub fn angle(&self) -> f32 {
        self.angle
    }
}
