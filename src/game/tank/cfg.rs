use config::{self, Config};
use ggez::graphics::Image;
use ggez::Context;
use std::convert::TryFrom;
use std::rc::Rc;

#[derive(Debug)]
pub struct TankCfgs {
    cfgs: Vec<Rc<TankCfg>>,
}

impl TankCfgs {
    pub fn load(ctx: &mut Context) -> TankCfgs {
        let cfgs = config::load("assets/config/tank.txt")
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

    pub fn get(&self, id: u32) -> Option<&Rc<TankCfg>> {
        self.cfgs.get(usize::try_from(id).unwrap())
    }
}

#[derive(Debug)]
pub struct TankCfg {
    pub id: u32,
    pub image: Image,
    pub barrel_image: Image,
    pub speed: u32,
    pub view: u32,
}

impl TankCfg {
    fn new(c: impl Config, ctx: &mut Context) -> Rc<TankCfg> {
        let id = c.u32("id").get();

        let image = c
            .str("image")
            .not_empty()
            .map(|s| Image::new(ctx, s.get()).expect(format!("{} not found", s).as_str()))
            .get();

        let barrel_image = c
            .str("barrel_image")
            .not_empty()
            .map(|s| Image::new(ctx, s.get()).expect(format!("{} not found", s).as_str()))
            .get();

        let speed = c.u32("speed").ge(1).get();

        Rc::new(TankCfg {
            id,
            image,
            barrel_image,
            speed,
            view: 500,
        })
    }
}
