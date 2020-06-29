pub struct Tank {
    pub x: u32,
    pub y: u32,
    pub vx: u32,
    pub vy: u32,
}

impl Tank {
    pub fn new() -> Tank {
        Tank {
            x: 0,
            y: 0,
            vx: 0,
            vy: 0,
        }
    }
}
