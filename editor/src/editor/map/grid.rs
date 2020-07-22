use self::cell::Cell;
use ggez::graphics::{self, Drawable};
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

    pub fn draw<'a, D, F>(&self, ctx: &mut Context, x: u32, y: u32, width: u32, height: u32, f: F)
    where
        D: Drawable + 'a,
        F: Fn(u8) -> Option<&'a D>,
    {
        // column
        let i1 = x / self.cell_size;
        if i1 >= self.cols {
            return;
        }

        let i2 = util::div_ceil(x + width, self.cell_size).min(self.cols);

        // row
        let j1 = y / self.cell_size;
        if j1 >= self.rows {
            return;
        }
        let j2 = util::div_ceil(y + height, self.cell_size).min(self.rows);

        // grid lines
        let y1 = util::substract(j1 * self.cell_size, y);
        let y2 = util::substract(j2 * self.cell_size, y);

        (i1..=i2)
            .map(|i| util::substract(i * self.cell_size, x))
            .for_each(|x| util::draw_line(ctx, x, y1, x, y2));

        let x1 = util::substract(i1 * self.cell_size, x);
        let x2 = util::substract(i2 * self.cell_size, x);

        (j1..=j2)
            .map(|j| util::substract(j * self.cell_size, y))
            .for_each(|y| util::draw_line(ctx, x1, y, x2, y));

        // grid images
        for i in i1..i2 {
            for j in j1..j2 {
                let v = self.cell(i, j).val;
                if let Some(d) = f(v) {
                    let x = util::substract(i * self.cell_size, x);
                    let y = util::substract(j * self.cell_size, y);
                    graphics::draw(ctx, d, ([x, y],)).unwrap();
                }
            }
        }
    }

    pub fn set(&mut self, x: u32, y: u32, val: u8) {
        let i = x / self.cell_size;
        let j = y / self.cell_size;
        if i < self.cols && j < self.rows {
            self.cell_set(i, j, val)
        }
    }

    fn cell_set(&mut self, i: u32, j: u32, val: u8) {
        self.cells[self.cell_idx(i, j)].val = val;
    }

    fn cell(&self, i: u32, j: u32) -> &Cell {
        &self.cells[self.cell_idx(i, j)]
    }

    fn cell_idx(&self, i: u32, j: u32) -> usize {
        (i * self.rows + j).try_into().unwrap()
    }
}
