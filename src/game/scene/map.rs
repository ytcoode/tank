use std::rc::Rc;

mod cfg;
pub use cfg::*;

pub struct Map {
    cfg: Rc<MapCfg>,
}

impl Map {
    pub fn new(cfg: Rc<MapCfg>) -> Self {
        Map { cfg }
    }
}
