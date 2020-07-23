use config::{self, Config};
use ggez::graphics::{spritebatch::SpriteBatch, Image};
use ggez::Context;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs;
use std::rc::Rc;
use util::byte::Bytes;
use util::file;
use util::push;

pub struct MapCfgs {
    map: HashMap<String, Rc<MapCfg>>,
}

impl MapCfgs {
    pub fn load(ctx: &mut Context) -> MapCfgs {
        let map = file::list("config/map/")
            .into_iter()
            .filter(|p| p.is_file())
            .map(|p| {
                let name = file::name(p);
                let data =
                    fs::read(p.as_path()).expect(format!("Failed to read file {:?}", p).as_str());
                (name, Rc::new(MapCfg::new(&name, ctx, data)))
            })
            .collect();

        MapCfgs { map }
    }

    pub fn get(&self, name: &str) -> Option<&Rc<MapCfg>> {
        self.map.get(name)
    }
}

pub struct MapCfg {
    tiles: Vec<Image>,
    tile_size: u32,
    rows: u32,
    cols: u32,
    grid: Vec<u8>,
    width: u32,
    height: u32,
}

// u8 tile_count
// for (tile_count) {
//   str tile_image
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

        for i in (0..tile_count) {
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
        let rows = b.read_u32();
        assert!(
            rows > 0,
            "The number of rows in the map grid must be greater than zero! {}",
            name
        );

        let cols = b.read_u32();
        assert!(
            cols > 0,
            "The number of columns in the map grid must be greater than zero! {}",
            name
        );

        let mut grid = Vec::with_capacity((rows * cols).try_into().unwrap());
        (0..grid.capacity())
            .map(|_| b.read_u8())
            .inspect(|&i| assert!((i as usize) < tile_images.len()))
            .for_each(|i| tiles.push(i));

        let width = cols * tile_size;
        let height = rows * tile_size;

        MapCfg {
            tiles,
            tile_size,
            rows,
            cols,
            grid,
            width,
            height,
        }
    }
}
