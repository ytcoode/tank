use self::grid::Grid;
use ggez::event::{self, EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics::{self, Image};
use ggez::{Context, GameResult};
use std::convert::TryInto;
use std::fs;
use util::byte::BytesMut;

mod grid;

pub struct Map {
    grid: Grid,
    tiles: Vec<(String, Image)>,
    tile: u8,
    view_x: i32,
    view_y: i32,
    view_drag: bool,
}

impl Map {
    pub fn new(ctx: &mut Context) -> Map {
        let grid = Grid::new(5, 10, 128); // the sizes of tile images are fixed, so we hardcode it here.

        let tiles = [
            "/a/PNG/Environment/dirt.png",
            "/a/PNG/Environment/grass.png",
            "/a/PNG/Environment/sand.png",
        ]
        .iter()
        .map(|p| {
            (
                p.to_string(),
                Image::new(ctx, p).expect("Failed to load image"),
            )
        })
        .inspect(|(p, i)| println!("tile loaded: {} -> {}*{}", p, i.width(), i.height()))
        .collect();

        Map {
            grid,
            tiles,
            tile: 1,
            view_x: 0,
            view_y: 0,
            view_drag: false,
        }
    }

    fn save(&mut self) {
        let mut b = Vec::new();

        // tile count
        b.write_u8(self.tiles.len().try_into().unwrap());

        // tiles
        self.tiles
            .iter()
            .map(|(s, _)| s)
            .for_each(|s| b.write_str(s));

        // grid
        self.grid.write_to(&mut b);

        // write to a file
        let path = "assets/config/map/1.map";
        fs::write(path, b.as_slice()).expect(format!("Could not write file: {}", path).as_str());
        println!("map saved successfully: {}", path);
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

        self.grid.draw(
            ctx,
            self.view_x,
            self.view_y,
            dw.ceil() as i32,
            dh.ceil() as i32,
            |v| match v {
                0 => None,
                _ => self.tiles.get(v as usize - 1).map(|t| &t.1),
            },
        );

        graphics::present(ctx)
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        match button {
            MouseButton::Left => {
                let x = self.view_x + x.round() as i32;
                let y = self.view_y + y.round() as i32;
                self.grid.set(x, y, self.tile);
            }
            MouseButton::Right => self.view_drag = true,
            _ => (),
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
        match button {
            MouseButton::Right => self.view_drag = false,
            _ => (),
        }
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, dx: f32, dy: f32) {
        if self.view_drag {
            self.view_x -= dx.round() as i32;
            self.view_y -= dy.round() as i32;
        }
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Key0 => self.tile = 0,
            KeyCode::Key1 => self.tile = 1,
            KeyCode::Key2 => self.tile = 2,
            KeyCode::Key3 => self.tile = 3,
            KeyCode::S => self.save(),
            KeyCode::Escape => event::quit(ctx),
            _ => (),
        }
    }
}
