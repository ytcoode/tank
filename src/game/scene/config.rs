use super::MapCfgs;
use crate::deps::config;
use crate::deps::config::Config;
use std::collections::HashMap;
use std::rc::Rc;

pub struct SceneCfgs {
    map: HashMap<u32, Rc<SceneCfg>>,
}

impl SceneCfgs {
    pub fn load(mapCfgs: &MapCfgs) -> SceneCfgs {
        let mut map = HashMap::new();

        config::load("config/scene.txt")
            .into_iter()
            .map(|c| Rc::new(SceneCfg::new(c, mapCfgs)))
            .map(|c| map.insert(c.id, c))
            .map(|o| o.map(|c| c.id))
            .for_each(|o| o.expect_none("Duplicate scene id"));

        SceneCfgs { map }
    }

    pub fn get(&self, id: u32) -> Option<&Rc<SceneCfg>> {
        self.map.get(&id)
    }
}

pub struct SceneCfg {
    pub id: u32,
    pub map: Rc<MapCfg>,
}

impl SceneCfg {
    fn new<C: Config>(c: C, mapCfgs: &MapCfgs) -> SceneCfg {
        let id = c.u32("id").get();

        let map = mapCfgs.get(c.str("map").not_empty().get()).clone();

        SceneCfg { id, map }
    }
}
