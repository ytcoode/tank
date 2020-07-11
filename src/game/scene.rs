use std::rc::Rc;

mod config;
pub use config::*;

pub struct Scene {
    cfg: Rc<SceneCfg>,
}
