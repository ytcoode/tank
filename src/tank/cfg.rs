use crate::config;
use crate::config::Config;
use ggez::graphics::Image;
use ggez::Context;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub struct TankCfgs {
    pub cfgs: Vec<Rc<TankCfg>>,
}

#[derive(Debug)]
pub struct TankCfg {
    id: u32,
    width: u32,
    height: u32,
    pub image: Image,
    pub ms: u16, // movement speed
}

impl TankCfg {
    fn new(c: impl Config, ctx: &mut Context) -> Rc<TankCfg> {
        let id = c.u32("id").get();

        let image = c
            .str("image")
            .not_empty()
            .map(|s| {
                Image::new(ctx, s.get())
                    .expect(format!("TankCfg{{id: {}}} image not found", id).as_str())
            })
            .get();

        let width = image.width().into();
        let height = image.height().into();

        let ms = c.u16("ms").ge(1).get();

        Rc::new(TankCfg {
            id,
            width,
            height,
            image,
            ms,
        })
    }
}

pub fn load_cfgs(ctx: &mut Context) -> TankCfgs {
    let cfgs = config::load("config/tank.txt")
        .into_iter()
        .map(|c| TankCfg::new(c, ctx))
        .enumerate()
        .map(|(i, t)| {
            assert_eq!(
                usize::try_from(t.id).unwrap(),
                i,
                "Tank id must start at zero and increase sequentially!"
            );
            t
        })
        .collect();

    TankCfgs { cfgs }
}

struct A {}
