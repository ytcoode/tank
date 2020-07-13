use super::map::MapCfg;
use std::rc::Rc;

// block width
// block height
// block state
// image one, xy, xy...
// image two, xy, xy...
//
pub struct Map {
    cfg: Rc<MapCfg>,
}
