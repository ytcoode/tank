#[derive(Debug, Clone)]
pub struct Tile {
    pub image: u8,
}

impl Tile {
    pub fn new() -> Tile {
        Tile { image: 0 }
    }
}
