use crate::deps::config;
use crate::deps::config::Config;
use crate::util::byte::Bytes;
use crate::util::file;
use crate::util::push;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;

pub struct MapCfgs {
    map: HashMap<String, Rc<MapCfg>>,
}

impl MapCfgs {
    pub fn load() -> MapCfgs {
        let map = file::list2("config/map/")
            .expect("Failed to list files in directory [config/map/]")
            .into_iter()
            .filter(|p| p.is_file())
            .map(|p| {
                (
                    p.file_name().unwrap().to_str().unwrap().to_string(),
                    Rc::new(MapCfg::new(
                        fs::read(p.as_path())
                            .expect(format!("Failed to read file {:?}", p).as_str()),
                    )),
                )
            })
            .collect();

        MapCfgs { map }
    }

    pub fn get(&self, name: &str) -> Option<&Rc<MapCfg>> {
        self.map.get(name)
    }
}

pub struct MapCfg {
    grid: Vec<u8>,
    grid_nx: u32,
    grid_ny: u32,
    scale: u32,
    width: u32,
    height: u32,
    // tiles: Vec<SpriteBatch>,
}

// gnx, gny, grid
// scale
impl MapCfg {
    fn new(v: Vec<u8>) -> MapCfg {
        let mut b = v.as_slice();

        let grid_nx = b.read_u32();
        let grid_ny = b.read_u32();

        let grid_nz = (grid_nx as usize).checked_mul(grid_ny as usize).unwrap();
        let grid = (0..grid_nz).fold(Vec::with_capacity(grid_nz), |v, _| push(v, b.read_u8()));
        assert!(grid.len() == grid.capacity());

        let scale = b.read_u32();
        let width = grid_nx.checked_mul(scale).unwrap(); // TODO
        let height = grid_ny.checked_mul(scale).unwrap();

        MapCfg {
            grid,
            grid_nx,
            grid_ny,
            scale,
            width,
            height,
        }
    }
}
