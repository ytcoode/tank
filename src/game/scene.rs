use self::map::Map;
use self::unit::Unit;
use crate::game::common::view::View;
use crate::game::tank::Tank;
use ggez::Context;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

pub use cfg::*;

mod cfg;
pub mod map;
pub mod unit;

pub struct Scene {
    cfg: Rc<SceneCfg>,
    map: Map,
    tanks: HashMap<u32, Rc<Tank>>,
    view: View,
}

impl Scene {
    pub fn new(cfg: Rc<SceneCfg>, ctx: &mut Context) -> Self {
        let map = Map::new(cfg.map.clone());
        let tanks = HashMap::new();

        let view = View::new(0, 0, ctx, &cfg.map);

        Scene {
            cfg,
            map,
            tanks,
            view,
        }
    }

    fn add_tank(&mut self, tank: Rc<Tank>) {
        self.tanks.insert(tank.id(), tank.clone()).unwrap();
        self.map.add(tank);
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        self.map.draw(ctx, &self.view);
    }
}
