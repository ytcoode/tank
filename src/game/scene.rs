use self::map::Map;
use self::unit::Unit;
use crate::game::tank::Tank;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

pub use cfg::*;

mod cfg;
mod map;
pub mod unit;

pub struct Scene {
    cfg: Rc<SceneCfg>,
    map: Map,
    tanks: HashMap<u32, Rc<Tank>>,
}

impl Scene {
    pub fn new(cfg: Rc<SceneCfg>) -> Self {
        let map = Map::new(cfg.map.clone());
        let tanks = HashMap::new();

        Scene { cfg, map, tanks }
    }

    fn add_tank(&mut self, tank: Rc<Tank>) {
        self.tanks.insert(tank.id(), tank.clone()).unwrap();
        self.map.add(tank);
    }
}
