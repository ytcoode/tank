use ggez;
use ggez::event;
use ggez::event::EventHandler;
use ggez::graphics;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::DrawParam;
use ggez::graphics::Image;
use ggez::input::keyboard::KeyCode;
use ggez::input::keyboard::KeyMods;
use ggez::nalgebra::Point2;
use ggez::nalgebra::Vector2;
use ggez::timer;
use ggez::Context;
use ggez::ContextBuilder;
use ggez::GameResult;

const WIDTH: u32 = 10000;
const HEIGHT: u32 = 10000;

struct Map {
    dirt_batch: SpriteBatch,
    grass_batch: SpriteBatch,
    sand_batch: SpriteBatch,
    tree_large_batch: SpriteBatch,
    tree_small_batch: SpriteBatch,
}

impl Map {
    fn new(ctx: &mut Context) -> GameResult<Map> {
        let dirt_batch = SpriteBatch::new(Image::new(ctx, "/PNG/Environment/dirt.png")?);
        let grass_batch = SpriteBatch::new(Image::new(ctx, "/PNG/Environment/grass.png")?);
        let sand_batch = SpriteBatch::new(Image::new(ctx, "/PNG/Environment/sand.png")?);
        let tree_large_batch = SpriteBatch::new(Image::new(ctx, "/PNG/Environment/treeLarge.png")?);
        let tree_small_batch = SpriteBatch::new(Image::new(ctx, "/PNG/Environment/treeSmall.png")?);

        Ok(Map {
            dirt_batch,
            grass_batch,
            sand_batch,
            tree_large_batch,
            tree_small_batch,
        })
    }
}
