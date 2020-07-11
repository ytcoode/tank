use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::Image;
use ggez::Context;
use ggez::GameResult;
use std::path::Path;

pub fn load_tile<P: AsRef<Path>>(ctx: &mut Context, path: P) -> GameResult<(SpriteBatch, u32)> {
    let image = Image::new(ctx, path)?;
    let size = image.width();
    assert_eq!(size, image.height());
    Ok((SpriteBatch::new(image), size as u32))
}

pub fn idx(i: u32, j: u32, ny: u32) -> usize {
    (i * ny + j) as usize
}
