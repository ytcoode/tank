use crate::util;
use ggez;
use ggez::graphics;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::Image;
use ggez::nalgebra::Point2;
use ggez::Context;
use ggez::GameResult;

const BLOCK_SIZE: u32 = 128;
const BLOCK_N_X: u32 = 10;
const BLOCK_N_Y: u32 = 10;
const BLOCK_N_XY: u32 = BLOCK_N_X * BLOCK_N_Y;

pub const MAP_WIDTH: f32 = (BLOCK_N_X * BLOCK_SIZE) as f32;
pub const MAP_HEIGHT: f32 = (BLOCK_N_Y * BLOCK_SIZE) as f32;

fn block_idx(i: u32, j: u32) -> usize {
    (i * BLOCK_N_Y + j) as usize
}

pub struct Map {
    map: [u8; BLOCK_N_XY as usize],
    dirt_batch: SpriteBatch,
    grass_batch: SpriteBatch,
    sand_batch: SpriteBatch,
    _tree_large_batch: SpriteBatch,
    _tree_small_batch: SpriteBatch,
}

impl Map {
    pub fn new(ctx: &mut Context) -> GameResult<Map> {
        let dirt_image = Image::new(ctx, "/PNG/Environment/dirt.png")?;
        assert_eq!(dirt_image.width() as u32, BLOCK_SIZE);
        assert_eq!(dirt_image.height() as u32, BLOCK_SIZE);
        let dirt_batch = SpriteBatch::new(dirt_image);

        let grass_image = Image::new(ctx, "/PNG/Environment/grass.png")?;
        assert_eq!(grass_image.width() as u32, BLOCK_SIZE);
        assert_eq!(grass_image.height() as u32, BLOCK_SIZE);
        let grass_batch = SpriteBatch::new(grass_image);

        let sand_image = Image::new(ctx, "/PNG/Environment/sand.png")?;
        assert_eq!(sand_image.width() as u32, BLOCK_SIZE);
        assert_eq!(sand_image.height() as u32, BLOCK_SIZE);
        let sand_batch = SpriteBatch::new(sand_image);

        let mut map = [0; BLOCK_N_XY as usize];
        let mut n = 0 as u32;
        for i in 0..BLOCK_N_X {
            for j in 0..BLOCK_N_Y {
                map[block_idx(i, j)] = (n % 3) as u8;
                n += 1;
            }
        }

        let _tree_large_batch =
            SpriteBatch::new(Image::new(ctx, "/PNG/Environment/treeLarge.png")?);
        let _tree_small_batch =
            SpriteBatch::new(Image::new(ctx, "/PNG/Environment/treeSmall.png")?);

        Ok(Map {
            map,
            dirt_batch,
            grass_batch,
            sand_batch,
            _tree_large_batch,
            _tree_small_batch,
        })
    }

    pub fn draw(&mut self, ctx: &mut Context, mut x: f32, mut y: f32) -> GameResult<(f32, f32)> {
        // 视野长宽
        let (vw, vh) = graphics::drawable_size(ctx);

        // 视野半径
        let vx = vw / 2.0;
        let vy = vh / 2.0;

        // 视野锚点
        if x < vx {
            x = vx;
        }

        if x > MAP_WIDTH - vx {
            x = (MAP_WIDTH - vx).max(vx);
        }

        if y < vy {
            y = vy;
        }

        if y > MAP_HEIGHT - vy {
            y = (MAP_HEIGHT - vy).max(vy);
        }

        // 视野范围
        let x1 = x - vx;
        let x2 = (x + vx).min(MAP_WIDTH);
        let y1 = y - vy;
        let y2 = (y + vy).min(MAP_HEIGHT);

        // 视野内的block
        let i1 = x1 as u32 / BLOCK_SIZE;
        let i2 = (x2 as u32 / BLOCK_SIZE).min(BLOCK_N_X - 1);
        let j1 = y1 as u32 / BLOCK_SIZE;
        let j2 = (y2 as u32 / BLOCK_SIZE).min(BLOCK_N_Y - 1);

        for i in i1..=i2 {
            for j in j1..=j2 {
                let xi = (i * BLOCK_SIZE) as f32 - x1;
                let yj = (j * BLOCK_SIZE) as f32 - y1;
                match self.map[block_idx(i, j)] {
                    0 => &mut self.dirt_batch,
                    1 => &mut self.grass_batch,
                    2 => &mut self.sand_batch,
                    _ => unreachable!(),
                }
                .add((Point2::new(xi, yj),));
            }
        }

        graphics::draw(ctx, &self.dirt_batch, util::DRAW_PARAM_ZERO)?;
        graphics::draw(ctx, &self.grass_batch, util::DRAW_PARAM_ZERO)?;
        graphics::draw(ctx, &self.sand_batch, util::DRAW_PARAM_ZERO)?;

        self.dirt_batch.clear();
        self.grass_batch.clear();
        self.sand_batch.clear();

        Ok((x1, y1))
    }
}
