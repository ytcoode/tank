use crate::deps::config;
use crate::deps::config::Config;
use std::collections::HashMap;

pub struct SceneCfgs {
    map: HashMap<u32, SceneCfg>,
}

impl SceneCfgs {
    fn load() -> SceneCfgs {
        let mut map = HashMap::new();

        config::load("config/scene.txt")
            .into_iter()
            .map(|i| SceneCfg::new(i))
            .map(|i| map.insert(i.id, i))
            .map(|i| i.map(|c| c.id))
            .for_each(|i| i.expect_none("Duplicate scene id"));

        SceneCfgs { map }
    }
}

pub struct SceneCfg {
    pub id: u32,
}

impl SceneCfg {
    fn new(c: impl Config) -> SceneCfg {
        let id = 0;

        SceneCfg { id }
    }
}
