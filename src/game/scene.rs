use self::map::Map;
use crate::game::tank::Tank;
use std::rc::Rc;

mod cfg;
mod map;
pub use cfg::*;

pub struct Scene {
    cfg: Rc<SceneCfg>,
    map: Map,
}

impl Scene {
    fn new(cfg: Rc<SceneCfg>) -> Self {
        let map = Map::new(cfg.map.clone());

        Scene { cfg, map }
    }
}
