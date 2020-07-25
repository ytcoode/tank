use ggez::graphics::Image;
use ggez::Context;

pub struct MiscCfgs {
    pub flag: Image,
}

impl MiscCfgs {
    pub fn load(ctx: &mut Context) -> MiscCfgs {
        let flag = Image::new(ctx, "/b/PNG/Black/1x/flag.png").expect("Image not found");

        MiscCfgs { flag }
    }
}
