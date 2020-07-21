use self::cell::Cell;
use ggez::graphics;
use ggez::Context;
use std::convert::TryInto;

mod cell;

pub struct Grid {
    rows: u32,
    cols: u32,
    cells: Vec<Cell>,
    cell_size: u32,
}

impl Grid {
    pub fn new(rows: u32, cols: u32, cell_size: u32) -> Self {
        let cells = vec![Cell::default(); (rows * cols).try_into().unwrap()];

        Grid {
            rows,
            cols,
            cells,
            cell_size,
        }
    }

    pub fn draw(&self, ctx: &mut Context, x: u32, y: u32, width: u32, height: u32) {}
}
