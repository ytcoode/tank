use self::grid::Grid;
use crate::game::scene::unit::Unit;
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
        let x = unit.x();
        let y = unit.y();

        let i = x / CELL_SIZE;
        let j = y / CELL_SIZE;

        self.grid.add(i, j, unit.clone());

        if let Some(v) = unit.view() {
            let x1 = x.saturating_sub(v.range);
            let x2 = (x + v.range).max(self.cfg.width);
            let y1 = y.saturating_sub(v.range);
            let y2 = (y + v.range).max(self.cfg.height);

            let i1 = x1 / CELL_SIZE;
            let i2 = util::div_ceil(x2, CELL_SIZE);
            let j1 = y1 / CELL_SIZE;
            let j2 = util::div_ceil(y2, CELL_SIZE);

            for i in i1..i2 {
                for j in j1..j2 {
                    self.grid.add_viewer(i, j, unit.clone());
                }
            }
            v.current_update(i1, i2, j1, j2);
        }
    }

    pub fn remove(&mut self, unit: &Rc<dyn Unit>) {
        let (i, j) = unit.map_cell().get();
        self.grid.remove(i, j, unit.id());

        if let Some(v) = unit.view() {
            let (i1, i2, j1, j2) = v.current();
            for i in i1..i2 {
                for j in j1..j2 {
                    self.grid.remove_viewer(i, j, unit.id());
                }
            }
        }
    }

    pub fn unit_moved(&mut self, unit: &Rc<dyn Unit>) {
        let x = unit.x();
        let y = unit.y();

        let i = x / CELL_SIZE;
        let j = y / CELL_SIZE;

        match unit.map_cell().get() {
            (i, j) => return, // position not changed
            (li, lj) => self.grid.unit_moved(li, lj, i, j, unit.id()),
        }

        if let Some(v) = unit.view() {
            let x1 = x.saturating_sub(v.range);
            let x2 = (x + v.range).max(self.cfg.width);
            let y1 = y.saturating_sub(v.range);
            let y2 = (y + v.range).max(self.cfg.height);

            let i1 = x1 / CELL_SIZE;
            let i2 = util::div_ceil(x2, CELL_SIZE);
            let j1 = y1 / CELL_SIZE;
            let j2 = util::div_ceil(y2, CELL_SIZE);

            match v.current() {
                (i1, i2, j1, j2) => (), // not changed
                (a1, a2, b1, b2) => {
                    for a in a1..a2 {
                        for b in b1..b2 {
                            if !util::is_inside_rectangle(i1, i2, j1, j2, a, b) {
                                self.grid.remove_viewer(i, j, unit.id());
                            }
                        }
                    }

                    for i in i1..i2 {
                        for j in j1..j2 {
                            if !util::is_inside_rectangle(a1, a2, b1, b2, i, j) {
                                self.grid.add_viewer(i, j, unit.clone());
                            }
                        }
                    }
                    v.current_update(i1, i2, j1, j2);
                }
            }
        }
    }
}
