use ggez;
use ggez::graphics;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::Image;
use ggez::nalgebra::Point2;
use ggez::Context;
use ggez::GameResult;
use rand;
use rand::Rng;
use std::cmp;

const TILE_SIZE: u32 = 128;
const TILE_NX: u32 = 100;
const TILE_NY: u32 = 100;

pub struct Map {
    map: [u8; (TILE_NX * TILE_NY) as usize],
    dirt_batch: SpriteBatch,
    grass_batch: SpriteBatch,
    sand_batch: SpriteBatch,
    _tree_large_batch: SpriteBatch,
    _tree_small_batch: SpriteBatch,
}

impl Map {
    pub fn new(ctx: &mut Context) -> GameResult<Map> {
        let dirt_image = Image::new(ctx, "/PNG/Environment/dirt.png")?;
        assert_eq!(dirt_image.width() as u32, TILE_SIZE);
        assert_eq!(dirt_image.height() as u32, TILE_SIZE);
        let dirt_batch = SpriteBatch::new(dirt_image);

        let grass_image = Image::new(ctx, "/PNG/Environment/grass.png")?;
        assert_eq!(grass_image.width() as u32, TILE_SIZE);
        assert_eq!(grass_image.height() as u32, TILE_SIZE);
        let grass_batch = SpriteBatch::new(grass_image);

        let sand_image = Image::new(ctx, "/PNG/Environment/sand.png")?;
        assert_eq!(sand_image.width() as u32, TILE_SIZE);
        assert_eq!(sand_image.height() as u32, TILE_SIZE);
        let sand_batch = SpriteBatch::new(sand_image);

        let mut map = [0; (TILE_NX * TILE_NY) as usize];
        let mut rng = rand::thread_rng();

        for i in 0..TILE_NX {
            for j in 0..TILE_NY {
                let ij = i * TILE_NY + j;
                map[ij as usize] = rng.gen_range(0, 3);
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

    pub fn draw(&mut self, ctx: &mut Context, x: f32, y: f32) -> GameResult {
        let (w, h) = graphics::drawable_size(ctx);

        let w = w / 2.0;
        let h = h / 2.0;

        let x1 = cmp::max((x - w) as i32, 0);
        let x2 = cmp::min((x + w) as i32, (TILE_NX * TILE_SIZE) as i32);
        let y1 = cmp::max((y - h) as i32, 0);
        let y2 = cmp::min((y + h) as i32, (TILE_NY * TILE_SIZE) as i32);

        let i1 = x1 as u32 / TILE_SIZE;
        let i2 = x2 as u32 / TILE_SIZE;
        let j1 = y1 as u32 / TILE_SIZE;
        let j2 = y2 as u32 / TILE_SIZE;

        let x0 = x - w;
        let y0 = y - h;

        for i in i1..=i2 {
            for j in j1..=j2 {
                let xi = (i * TILE_SIZE) as f32 - x0;
                let yj = (j * TILE_SIZE) as f32 - y0;
                let ij = i * TILE_NY + j;
                match self.map[ij as usize] {
                    0 => &mut self.dirt_batch,
                    1 => &mut self.grass_batch,
                    2 => &mut self.sand_batch,
                    _ => unreachable!(),
                }
                .add((Point2::new(xi, yj),));
            }
        }

        graphics::draw(ctx, &self.dirt_batch, (Point2::new(0.0, 0.0),))?;
        graphics::draw(ctx, &self.grass_batch, (Point2::new(0.0, 0.0),))?;
        graphics::draw(ctx, &self.sand_batch, (Point2::new(0.0, 0.0),))?;

        self.dirt_batch.clear();
        self.grass_batch.clear();
        self.sand_batch.clear();

        Ok(())
    }
}
