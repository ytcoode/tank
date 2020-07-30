use super::map::{MapCfg, MapCfgs};
use config::{self, Config};
use ggez::Context;
use std::collections::HashMap;
use std::rc::Rc;

pub struct SceneCfgs {
    map: HashMap<u32, Rc<SceneCfg>>,
}

impl SceneCfgs {
    pub fn load(ctx: &mut Context) -> SceneCfgs {
        let mapCfgs = MapCfgs::load(ctx);
        let mut map = HashMap::new();

        config::load("config/scene.txt")
            .into_iter()
            .map(|c| Rc::new(SceneCfg::new(c, &mapCfgs)))
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
        let map = c
            .str("map")
            .not_empty()
            .map(|s| {
                mapCfgs
                    .get(s.get())
                    .expect(format!("{} not found", s).as_str())
                    .clone()
            })
            .get();

        SceneCfg { id, map }
    }
}
