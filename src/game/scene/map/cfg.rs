use config::{self, Config};
use ggez::graphics::{spritebatch::SpriteBatch, Image};
use ggez::Context;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs;
use std::rc::Rc;
use util::byte::Bytes;
use util::{file, push};

pub struct MapCfgs {
    map: HashMap<String, Rc<MapCfg>>,
}

impl MapCfgs {
    pub fn load(ctx: &mut Context) -> MapCfgs {
        let map = file::list("assets/config/map/")
            .into_iter()
            .filter(|p| p.is_file())
            .map(|p| {
                let name = file::name(&p);
                let data =
                    fs::read(p.as_path()).expect(format!("Failed to read file {:?}", p).as_str());
                let cfg = Rc::new(MapCfg::new(&name, data, ctx));
                (name, cfg)
            })
            .collect();

        MapCfgs { map }
    }

    pub fn get(&self, name: &str) -> Option<&Rc<MapCfg>> {
        self.map.get(name)
    }
}

pub struct MapCfg {
    pub tiles: Vec<Image>,
    pub tile_size: u32,
    pub tile_rows: u32,
    pub tile_cols: u32,
    pub tile_grid: Vec<u8>, // tile index
    pub width: u32,
    pub height: u32,
}

// u8 tile_count
// for (tile_count) {
//   str tiles
// }
//
// u32 rows
// u32 cols
// for (rows * cols) {
//   u8 tile_idx
// }
impl MapCfg {
    fn new(name: &str, v: Vec<u8>, ctx: &mut Context) -> MapCfg {
        let mut b = v.as_slice();

        // tiles
        let tile_count = b.read_u8();
        assert!(
            tile_count > 0,
            "The number of tiles must be greater than zero! {}",
            name,
        );

        let mut tiles = Vec::with_capacity(tile_count.into());
        let mut tile_size = 0;

        for i in 0..tile_count {
            let tile = b.read_str();
            let image = Image::new(ctx, tile).expect("Failed to load tile image");

            assert!(
                image.width() == image.height(),
                "The width and height of tiles must be equal! {}: {}",
                name,
                tile,
            );

            assert!(
                image.width() > 0,
                "The tile size must be greater than zero! {}: {}",
                name,
                tile,
            );

            if i == 0 {
                tile_size = image.width() as u32;
            } else {
                assert_eq!(
                    tile_size,
                    image.width() as u32,
                    "All tiles must be of the same size! {}",
                    name
                );
            }

            tiles.push(image);
        }

        assert!(tile_size > 0);

        // rows & cols
        let tile_rows = b.read_u32();
        assert!(
            tile_rows > 0,
            "The number of rows in the map grid must be greater than zero! {}",
            name
        );

        let tile_cols = b.read_u32();
        assert!(
            tile_cols > 0,
            "The number of columns in the map grid must be greater than zero! {}",
            name
        );

        let mut tile_grid = Vec::with_capacity((tile_rows * tile_cols).try_into().unwrap());
        (0..tile_grid.capacity())
            .map(|_| b.read_u8())
            .inspect(|&i| {
                assert!(
                    (i as usize) < tiles.len(),
                    "tile index out of bound: {}, {}",
                    i,
                    tiles.len()
                )
            })
            .for_each(|i| tile_grid.push(i));

        let width = tile_cols * tile_size;
        let height = tile_rows * tile_size;

        MapCfg {
            tiles,
            tile_size,
            tile_rows,
            tile_cols,
            tile_grid,
            width,
            height,
        }
    }

    pub fn tile_idx(&self, i: u32, j: u32) -> usize {
        self.tile_grid[self.grid_idx(i, j)] as usize
    }

    fn grid_idx(&self, i: u32, j: u32) -> usize {
        (i * self.tile_rows + j).try_into().unwrap()
    }
}
