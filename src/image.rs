use ggez::graphics::Image;
use ggez::Context;

pub struct Images {
    pub flag: Image,
}

impl Images {}

pub fn load(ctx: &mut Context) -> Images {
    let flag = Image::new(ctx, "/b/PNG/Black/1x/flag.png").expect("Image not found");

    Images { flag }
}
