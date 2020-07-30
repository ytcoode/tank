use super::super::Unit;
use std::collections::HashMap;
use std::convert::TryInto;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub struct Grid {
    rows: u32,
    cols: u32,
    cells: Vec<Cell>,
}

impl Grid {
    pub fn new(rows: u32, cols: u32) -> Self {
        let cells = vec![Cell::default(); (rows * cols).try_into().unwrap()];
        Grid { rows, cols, cells }
    }

    pub fn add(&mut self, i: u32, j: u32, unit: Rc<dyn Unit>) {
        self.cell_mut(i, j).insert(unit.id(), unit).unwrap_none();
    }

    fn cell(&self, i: u32, j: u32) -> &Cell {
        &self.cells[self.cell_idx(i, j)]
    }

    fn cell_mut(&mut self, i: u32, j: u32) -> &mut Cell {
        let k = self.cell_idx(i, j);
        &mut self.cells[k]
    }

    fn cell_idx(&self, i: u32, j: u32) -> usize {
        (i * self.rows + j).try_into().unwrap()
    }
}

#[derive(Clone, Default)]
pub struct Cell {
    units: HashMap<u32, Rc<dyn Unit>>,
}

impl Deref for Cell {
    type Target = HashMap<u32, Rc<dyn Unit>>;

    fn deref(&self) -> &Self::Target {
        &self.units
    }
}

impl DerefMut for Cell {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.units
    }
}
