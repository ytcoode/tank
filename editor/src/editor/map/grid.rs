use self::cell::Cell;
use ggez::graphics;
use ggez::Context;
use std::convert::TryInto;
use util;

mod cell;

pub struct Grid {
    rows: u32,
    cols: u32,
    cells: Vec<Cell>,
    cell_size: u32,
}

impl Grid {
    pub fn new(rows: u32, cols: u32, cell_size: u32) -> Self {
        assert!(rows > 0 && cols > 0);
        let cells = vec![Cell::default(); (rows * cols).try_into().unwrap()];

        Grid {
            rows,
            cols,
            cells,
            cell_size,
        }
    }

    pub fn draw(&self, ctx: &mut Context, x: u32, y: u32, width: u32, height: u32) {
        // column
        let c1 = x / self.cell_size;
        if c1 >= self.cols {
            return;
        }
        let c2 = ((x + width) / self.cell_size).min(self.cols - 1);

        // row
        let r1 = y / self.cell_size;
        if r1 >= self.rows {
            return;
        }
        let r2 = ((y + height) / self.cell_size).min(self.rows - 1);

        // grid
        (c1..=c2).for_each(|c| {
            let cx = c * self.cell_size;
            util::draw_line(ctx, cx, y, cx, y + height);
        });
    }
}
