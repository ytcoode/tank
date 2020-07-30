use self::grid::Grid;
use super::Unit;
use crate::game::tank::Tank;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::rc::Rc;
use util;

mod cfg;
pub use cfg::*;

mod grid;

const GRID_CELL_SIZE: u32 = 256;

fn grid_idx(a: u32) -> u32 {
    a / GRID_CELL_SIZE
}

pub struct Map {
    cfg: Rc<MapCfg>,
    grid: Grid,
}

impl Map {
    pub fn new(cfg: Rc<MapCfg>) -> Self {
        let rows = util::div_ceil(cfg.height, GRID_CELL_SIZE);
        let cols = util::div_ceil(cfg.width, GRID_CELL_SIZE);
        let grid = Grid::new(rows, cols);

        Map { cfg, grid }
    }

    pub fn add(&mut self, unit: Rc<dyn Unit>) {
        let i = grid_idx(unit.x());
        let j = grid_idx(unit.y());
        self.grid.add(i, j, unit);
    }
}
