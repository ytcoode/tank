use self::cell::Cell;
use ggez::graphics::{self, Drawable};
use ggez::Context;
use std::convert::TryInto;
use util;
use util::byte::BytesMut;

mod cell;

pub struct Grid {
    rows: i32,
    cols: i32,
    cells: Vec<Cell>,
    cell_size: i32,
    width: i32,
    height: i32,
}

impl Grid {
    pub fn new(rows: i32, cols: i32, cell_size: i32) -> Self {
        assert!(rows > 0);
        assert!(cols > 0);
        assert!(cell_size > 0);

        let cells = vec![Cell::default(); (rows * cols).try_into().unwrap()];

        Grid {
            rows,
            cols,
            cells,
            cell_size,
            width: cols * cell_size,
            height: rows * cell_size,
        }
    }

    pub fn write_to<B: BytesMut>(&self, b: &mut B) {
        // rows & cols
        b.write_u32(self.rows as u32);
        b.write_u32(self.cols as u32);

        // // cell size
        // b.write_u32(self.cell_size as u32);

        // cells
        self.cells
            .iter()
            .map(|c| c.val)
            .inspect(|&v| assert!(v > 0, "The grid cell cannot be empty!"))
            .for_each(|v| b.write_u8(v));
    }

    pub fn draw<'a, D, F>(&self, ctx: &mut Context, x: i32, y: i32, width: i32, height: i32, f: F)
    where
        D: Drawable + 'a,
        F: Fn(u8) -> Option<&'a D>,
    {
        assert!(width > 0);
        assert!(height > 0);

        // x1 & x2
        if x >= self.width {
            return;
        }

        let x2 = x + width;
        if x2 <= 0 {
            return;
        }

        let x1 = x.max(0);
        let x2 = x2.min(self.width);

        // y1 & y2
        if y >= self.height {
            return;
        }

        let y2 = y + height;
        if y2 <= 0 {
            return;
        }

        let y1 = y.max(0);
        let y2 = y2.min(self.height);

        // i1 & i2
        let i1 = x1 / self.cell_size;
        let i2 = (x2 + self.cell_size - 1) / self.cell_size;

        // j1 & j2
        let j1 = y1 / self.cell_size;
        let j2 = (y2 + self.cell_size - 1) / self.cell_size;

        // grid lines
        (i1..=i2).map(|i| i * self.cell_size - x).for_each(|x| {
            util::draw_line(ctx, x as f32, (y1 - y) as f32, x as f32, (y2 - y) as f32)
        });

        (j1..=j2).map(|j| j * self.cell_size - y).for_each(|y| {
            util::draw_line(ctx, (x1 - x) as f32, y as f32, (x2 - x) as f32, y as f32)
        });

        // grid cells
        for i in i1..i2 {
            for j in j1..j2 {
                let v = self.cell(i, j).val;
                if let Some(d) = f(v) {
                    graphics::draw(
                        ctx,
                        d,
                        ([
                            (i * self.cell_size - x) as f32,
                            (j * self.cell_size - y) as f32,
                        ],),
                    )
                    .unwrap();
                }
            }
        }
    }

    pub fn set(&mut self, x: i32, y: i32, val: u8) {
        let i = x / self.cell_size;
        let j = y / self.cell_size;
        if i >= 0 && i < self.cols && j >= 0 && j < self.rows {
            self.cell_set(i, j, val)
        }
    }

    fn cell(&self, i: i32, j: i32) -> &Cell {
        &self.cells[self.cell_idx(i, j)]
    }

    fn cell_set(&mut self, i: i32, j: i32, val: u8) {
        let idx = self.cell_idx(i, j);
        self.cells[idx].val = val;
    }

    fn cell_idx(&self, i: i32, j: i32) -> usize {
        (i * self.rows + j).try_into().unwrap()
    }
}
