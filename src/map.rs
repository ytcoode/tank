use ggez;
use ggez::graphics;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::Image;
use ggez::nalgebra::Point2;
use ggez::Context;
use ggez::GameResult;
use rand;
use rand::Rng;

const TILE_SIZE: u32 = 128;
const TILE_NX: u32 = 100;
const TILE_NY: u32 = 100;

pub struct Map {
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
        let mut dirt_batch = SpriteBatch::new(dirt_image);

        let grass_image = Image::new(ctx, "/PNG/Environment/grass.png")?;
        assert_eq!(grass_image.width() as u32, TILE_SIZE);
        assert_eq!(grass_image.height() as u32, TILE_SIZE);
        let mut grass_batch = SpriteBatch::new(grass_image);

        let sand_image = Image::new(ctx, "/PNG/Environment/sand.png")?;
        assert_eq!(sand_image.width() as u32, TILE_SIZE);
        assert_eq!(sand_image.height() as u32, TILE_SIZE);
        let mut sand_batch = SpriteBatch::new(sand_image);

        let mut rng = rand::thread_rng();

        for i in 0..TILE_NX {
            let x = i * TILE_SIZE;
            for j in 0..TILE_NY {
                let y = j * TILE_SIZE;
                match rng.gen_range(0, 3) {
                    0 => &mut dirt_batch,
                    1 => &mut grass_batch,
                    2 => &mut sand_batch,
                    _ => unreachable!(),
                }
                .add((Point2::new(x as f32, y as f32),));
            }
        }

        let _tree_large_batch =
            SpriteBatch::new(Image::new(ctx, "/PNG/Environment/treeLarge.png")?);
        let _tree_small_batch =
            SpriteBatch::new(Image::new(ctx, "/PNG/Environment/treeSmall.png")?);

        Ok(Map {
            dirt_batch,
            grass_batch,
            sand_batch,
            _tree_large_batch,
            _tree_small_batch,
        })
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::draw(ctx, &self.dirt_batch, (Point2::new(0.0, 0.0),))?;
        graphics::draw(ctx, &self.grass_batch, (Point2::new(0.0, 0.0),))?;
        graphics::draw(ctx, &self.sand_batch, (Point2::new(0.0, 0.0),))
    }
}
