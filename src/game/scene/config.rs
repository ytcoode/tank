use crate::deps::config;
use crate::deps::config::Config;
use std::collections::HashMap;
use std::rc::Rc;

pub struct SceneCfgs {
    map: HashMap<u32, Rc<SceneCfg>>,
}

impl SceneCfgs {
    pub fn load() -> SceneCfgs {
        let mut map = HashMap::new();

        config::load("config/scene.txt")
            .into_iter()
            .map(|i| Rc::new(SceneCfg::new(i)))
            .map(|i| map.insert(i.id, i))
            .map(|i| i.map(|c| c.id))
            .for_each(|i| i.expect_none("Duplicate scene id"));

        SceneCfgs { map }
    }

    pub fn get(&self, id: u32) -> Option<&Rc<SceneCfg>> {
        self.map.get(&id)
    }
}

pub struct SceneCfg {
    pub id: u32,
    //    pub map: Ref<u32, MapCfg>,
}

impl SceneCfg {
    fn new(c: impl Config) -> SceneCfg {
        let id = c.u32("id").get();

        SceneCfg { id }
    }
}
