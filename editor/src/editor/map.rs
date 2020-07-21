use self::grid::Grid;
use ggez::event::EventHandler;
use ggez::graphics::{self, Image};
use ggez::{Context, GameResult};

mod grid;

pub struct Map {
    grid: Grid,
    tiles: Vec<Image>,
}

impl Map {
    pub fn new(ctx: &mut Context) -> Map {
        let grid = Grid::new(4, 3, 128);

        let tiles = [
            "/a/PNG/Environment/dirt.png",
            "/a/PNG/Environment/grass.png",
            "/a/PNG/Environment/sand.png",
        ]
        .into_iter()
        .map(|p| Image::new(ctx, p).expect("Failed to load image"))
        .inspect(|i| println!("tile loaded: {}*{}", i.width(), i.height()))
        .collect();

        Map { grid, tiles }
    }
}

impl EventHandler for Map {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        let (dw, dh) = graphics::drawable_size(ctx);

        self.grid
            .draw(ctx, 0, 0, dw.ceil() as u32, dw.ceil() as u32);

        graphics::present(ctx)
    }
}
