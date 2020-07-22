use self::grid::Grid;
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::{self, Image};
use ggez::{Context, GameResult};

mod grid;

pub struct Map {
    grid: Grid,
    tiles: Vec<Image>,
    tile: u8,
    view_x: u32,
    view_y: u32,
}

impl Map {
    pub fn new(ctx: &mut Context) -> Map {
        let grid = Grid::new(4, 3, 128);

        let tiles = [
            "/a/PNG/Environment/dirt.png",
            "/a/PNG/Environment/grass.png",
            "/a/PNG/Environment/sand.png",
        ]
        .iter()
        .map(|p| Image::new(ctx, p).expect("Failed to load image"))
        .inspect(|i| println!("tile loaded: {}*{}", i.width(), i.height()))
        .collect();

        Map {
            grid,
            tiles,
            tile: 1,
            view_x: 0,
            view_y: 0,
        }
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
            .draw(ctx, 0, 0, dw.ceil() as u32, dh.ceil() as u32);

        graphics::present(ctx)
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        let mut x = x.round() as u32;
        let mut y = y.round() as u32;
        x += self.view_x;
        y += self.view_y;
        self.grid.set(x, y, self.tile);
    }
}
