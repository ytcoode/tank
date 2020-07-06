use crate::config;
use crate::config::Config;
use ggez::graphics::Image;
use ggez::Context;
use std::fmt;

pub fn load_cfgs(ctx: &mut Context) -> TankCfgs {
    let cfgs = config::load("config/tank.txt")
        .into_iter()
        .map(|c| TankCfg::new(c, ctx))
        .enumerate()
        .inspect(|(i, t)| {
            assert_eq!(
                t.id, i,
                "Tank id must start at zero and increase sequentially!"
            )
        })
        .map(|(_, t)| t)
        .collect();

    TankCfgs { cfgs }
}

#[derive(Debug)]
pub struct TankCfgs {
    cfgs: Vec<TankCfg>,
}

impl TankCfgs {
    pub fn get(&self, id: u32) -> Option<&TankCfg> {
        self.map.get(&id)
    }
}

#[derive(Debug)]
pub struct TankCfg {
    id: u32,
    width: u32,
    height: u32,
    image: Image,
}

impl TankCfg {
    fn new(c: impl Config, ctx: &mut Context) -> TankCfg {
        let id = c.u32("id").get();

        let image = c
            .str("image")
            .not_empty()
            .map(|s| {
                Image::new(ctx, s.get())
                    .expect(format!("TankCfg{{id: {}}} image not found", id).as_str())
            })
            .get();

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

impl fmt::Display for TankCfg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TankCfg").field("id", &self.id).finish()
    }
}
