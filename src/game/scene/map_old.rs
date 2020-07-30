use crate::util;
use ggez;
use ggez::graphics;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::Context;
use ggez::GameResult;
use std::convert::TryInto;

mod block;
mod config;
pub use config::*;

pub struct Map {
    blocks: Vec<u8>,
    block_nx: u32,
    block_ny: u32,

    block_tiles: Vec<SpriteBatch>,
    block_size: u32,

    width: u32,
    height: u32,
}

impl Map {
    pub fn new(ctx: &mut Context) -> GameResult<Map> {
        let tiles = [
            "/a/PNG/Environment/dirt.png",
            "/a/PNG/Environment/grass.png",
            "/a/PNG/Environment/sand.png",
        ];

        assert!(tiles.len() <= 0xf);
        let mut block_tiles = Vec::with_capacity(tiles.len());
        let mut block_size = 0;

        for tile in &tiles {
            let (a, b) = block::load_tile(ctx, tile)?;
            if block_size > 0 {
                assert_eq!(block_size, b);
            }
            block_tiles.push(a);
            block_size = b;
        }

        let block_nx = 10 as u32;
        let block_ny = 8 as u32;
        let mut blocks = Vec::with_capacity((block_nx * block_ny) as usize);
        for _ in 0..block_nx {
            for _ in 0..block_ny {
                blocks.push((blocks.len() % block_tiles.len()).try_into().unwrap());
            }
        }

        let width = block_nx * block_size;
        let height = block_ny * block_size;

        Ok(Map {
            blocks,
            block_nx,
            block_ny,

            block_tiles,
            block_size,

            width,
            height,
        })
    }

    pub fn draw(&mut self, ctx: &mut Context, x1: u32, x2: u32, y1: u32, y2: u32) -> GameResult {
        let i1 = x1 / self.block_size;
        let i2 = (x2 / self.block_size).min(self.block_nx - 1);
        let j1 = y1 / self.block_size;
        let j2 = (y2 / self.block_size).min(self.block_ny - 1);

        for i in i1..=i2 {
            for j in j1..=j2 {
                let x = (i * self.block_size) as f32 - x1 as f32;
                let y = (j * self.block_size) as f32 - y1 as f32;
                let t = self.block(i, j) as usize;
                self.block_tiles[t].add(([x, y],));
            }
        }

        for b in self.block_tiles.iter_mut() {
            graphics::draw(ctx, b, util::DRAW_PARAM_ZERO)?;
            b.clear();
        }

        Ok(())
    }

    fn block(&self, i: u32, j: u32) -> u8 {
        self.blocks[block::idx(i, j, self.block_ny)]
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
