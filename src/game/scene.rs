use crate::game::tank::Tank;
use std::rc::Rc;

mod config;
pub use config::*;

pub mod map;
use map::{Map, MapCfg};

mod map2;

pub struct Scene {
    cfg: Rc<SceneCfg>,
    map: Map,
    tanks: Vec<Rc<Tank>>,
    tank: Rc<Tank>,
}

impl Scene {
    //    pub fn new(cfg: Rc<SceneCfg>, tankcfg: Rc<TankCfg>, x: u32, y: u32) -> Scene {}
}
