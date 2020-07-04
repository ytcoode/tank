use crate::config;
use crate::config::Config;
use ggez::graphics::Image;
use ggez::Context;
use std::collections::HashMap;
use std::fmt;

pub struct TankCfgs {
    map: HashMap<u32, TankCfg>,
}

struct TankCfg {
    id: u32,
    width: u32,
    height: u32,
    image: Image,
}

impl TankCfg {
    fn new<C: Config>(c: C, ctx: &mut Context) -> TankCfg {
        let id = c.str("id");
        let id = id
            .parse()
            .expect(format!("Failed to parse {} as tank id", id).as_str());

        let image = c.str("image");
        let image = Image::new(ctx, image).expect("Image not found");

        let width = image.width() as u32;
        let height = image.height() as u32;

        TankCfg {
            id,
            width,
            height,
            image,
        }
    }
}

impl fmt::Debug for TankCfg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TankCfg").field("id", &self.id).finish()
    }
}

pub fn load_cfgs(ctx: &mut Context) -> TankCfgs {
    let mut map = HashMap::new();

    config::load("config/tank.txt")
        .into_iter()
        .map(|c| TankCfg::new(c, ctx))
        .for_each(|t| map.insert(t.id, t).expect_none("Duplicate tank id"));

    TankCfgs { map }
}
