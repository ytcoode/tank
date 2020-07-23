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
// u8 tile_count
// for (tile_count) {
//   str tile_image
// }
//
// for (tile_nx * tile_ny) {
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
        let tile_count = b.read_u8();
        assert!(
            tile_count > 0,
            "The number of tiles must be greater than zero! {}",
            name,
        );

        let mut tile_images = Vec::with_capacity(tile_count.into());
        let mut tile_size = 0;

        for i in (0..tile_count) {
            let tile = b.read_str();
            let image = Image::new(ctx, tile).expect("Failed to load image");

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

            tile_images.push(image);
        }

        assert!(tile_size > 0);
        assert!(
            width % tile_size == 0,
            "The map width ({}) must be divisible by tile size ({})! {}",
            width,
            tile_size,
            name
        );
        assert!(
            height % tile_size == 0,
            "The map height ({}) must be divisible by tile size ({})! {}",
            height,
            tile_size,
            name
        );

        let tile_nx = width / tile_size;
        let tile_ny = height / tile_size;
        let tile_nz = tile_nx * tile_ny;

        let mut tiles = Vec::with_capacity(tile_nz.try_into().unwrap());
        (0..tile_nz)
            .map(|_| b.read_u8())
            .inspect(|&i| assert!((i as usize) < tile_images.len()))
            .for_each(|i| tiles.push(i));

        // extra tiles
        let mut tile_extra_images = Vec::new();
        let mut tile_extra_positions = Vec::new();

        loop {
            let tile = b.read_str();
            let image = Image::new(ctx, tile).expect("Failed to load image");

            let count = b.read_u32();
            let mut positions = Vec::with_capacity(count.try_into().unwrap());

            (0..count)
                .map(|_| (b.read_u32(), b.read_u32()))
                .for_each(|p| positions.push(p));

            tile_extra_images.push(image);
            tile_extra_positions.push(positions);
        }

        tile_extra_images.shrink_to_fit();
        tile_extra_positions.shrink_to_fit();

        MapCfg {
            grid,
            grid_nx,
            grid_ny,
            scale,
            width,
            height,

            tiles,
            tile_nx,
            tile_ny,
            tile_size,
            tile_images,

            tile_extra_images,
            tile_extra_positions,
        }
    }
}
