pub struct Tank {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
}

impl Tank {
    pub fn new() -> Tank {
        Tank {
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
        }
    }
}
