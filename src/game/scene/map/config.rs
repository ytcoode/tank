use crate::deps::config::{self, Config};
use crate::util::byte::Bytes;
use crate::util::file;
use crate::util::push;
use ggez::graphics::{spritebatch::SpriteBatch, Image};
use ggez::Context;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs;
use std::rc::Rc;

pub struct MapCfgs {
    map: HashMap<String, Rc<MapCfg>>,
}

impl MapCfgs {
    pub fn load(ctx: &mut Context) -> MapCfgs {
        let map = file::list2("config/map/")
            .expect("Failed to list files in directory [config/map/]")
            .into_iter()
            .filter(|p| p.is_file())
            .map(|p| {
                let name = p.file_name().unwrap().to_str().unwrap().to_string();
                let cfg = Rc::new(MapCfg::new(
                    &name,
                    fs::read(p.as_path()).expect(format!("Failed to read file {:?}", p).as_str()),
                    ctx,
                ));
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
    // map grid
    grid: Vec<u8>,
    grid_nx: u32,
    grid_ny: u32,

    scale: u32,
    width: u32,
    height: u32,

    // tile grid
    tiles: Vec<u8>,
    tile_nx: u32,
    tile_ny: u32,
    tile_size: u32,
    tile_images: Vec<Image>,

    // extra tiles & positions
    tile_extra_images: Vec<Image>,
    tile_extra_positions: Vec<Vec<(u32, u32)>>,
}

// u32 grid_nx
// u32 grid_ny
// for (grid_nx * grid_ny) {
//   u8 grid_data
// }
//
// u32 scale
//
// u32 tile_count
// for (tile_count) {
//   str tile_image
// }
//
// u32 tile_index_count
// for (tile_index_count) {
//   u8 tile_index
// }
//
// while (readable) {
//   str extra_tile
//   u32 extra_tile_position_count
//   for (extra_tile_position_count) {
//     u32 extra_tile_x
//     u32 extra_tile_y
//   }
// }

impl MapCfg {
    fn new(name: &str, v: Vec<u8>, ctx: &mut Context) -> MapCfg {
        let mut b = v.as_slice();

        // grid
        let grid_nx = b.read_u32();
        assert!(
            grid_nx > 0,
            "The number of columns in the map grid must be greater than zero! {}",
            name
        );

        let grid_ny = b.read_u32();
        assert!(
            grid_ny > 0,
            "The number of rows in the map grid must be greater thant zero! {}",
            name
        );

        let grid_nz = (grid_nx * grid_ny).try_into().unwrap();
        let grid = (0..grid_nz).fold(Vec::with_capacity(grid_nz), |v, _| push(v, b.read_u8()));
        assert!(grid.len() == grid.capacity());

        // scale
        let scale = b.read_u32();
        assert!(
            scale > 0,
            "The scale of the map must be greater than zero! {}",
            name
        );

        let width = grid_nx * scale;
        let height = grid_ny * scale;

        // tiles
        loop {
            let tile = b.read_str();
            let image = Image::new(ctx, tile).expect("TODO");
            let sprite = SpriteBatch::new(image);

            let n = b.read_u32();
            while n > 0 {
                n -= 1;
                let x = b.read_u32();
                let y = b.read_u32();
            }
        }

        MapCfg {
            name,
            grid,
            grid_nx,
            grid_ny,
            scale,
            width,
            height,
        }
    }
}
