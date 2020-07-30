use config::{self, Config};
use ggez::graphics::Image;
use ggez::Context;
use std::rc::Rc;

#[derive(Debug)]
pub struct TankCfgs {
    pub cfgs: Vec<Rc<TankCfg>>,
}

impl TankCfgs {
    pub fn load(ctx: &mut Context) -> TankCfgs {
        let cfgs = config::load("assets/config/tank.txt")
            .into_iter()
            .map(|c| TankCfg::new(c, ctx))
            .enumerate()
            .map(|(i, t)| {
                assert_eq!(
                    usize::from(t.id),
                    i,
                    "Tank id must start at zero and increase sequentially!"
                );
                t
            })
            .collect();

        TankCfgs { cfgs }
    }

    pub fn get(&self, id: u16) -> Option<&Rc<TankCfg>> {
        self.cfgs.get(usize::from(id))
    }
}

#[derive(Debug)]
pub struct TankCfg {
    pub id: u16,
    pub image: Image,
    pub barrel_image: Image,
    pub speed: u16,
}

impl TankCfg {
    fn new(c: impl Config, ctx: &mut Context) -> Rc<TankCfg> {
        let id = c.u16("id").get();

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

        let speed = c.u16("speed").ge(1).get();

        Rc::new(TankCfg {
            id,
            image,
            barrel_image,
            speed,
        })
    }
}
