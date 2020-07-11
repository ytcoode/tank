use std::rc::Rc;

mod config;
pub use config::*;

pub mod map; // TODO: remove the pub keyword

pub struct Scene {
    cfg: Rc<SceneCfg>,
}
