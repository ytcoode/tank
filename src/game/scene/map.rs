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

const CELL_SIZE: u32 = 256;

pub struct Map {
    cfg: Rc<MapCfg>,
    grid: Grid,
}

impl Map {
    pub fn new(cfg: Rc<MapCfg>) -> Self {
        let rows = util::div_ceil(cfg.height, CELL_SIZE);
        let cols = util::div_ceil(cfg.width, CELL_SIZE);
        let grid = Grid::new(rows, cols);

        Map { cfg, grid }
    }

    pub fn add(&mut self, unit: Rc<dyn Unit>) {
        let i = unit.x() / CELL_SIZE;
        let j = unit.y() / CELL_SIZE;
        self.grid.add(i, j, unit);
    }
}
