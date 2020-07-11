use crate::deps::config;
use crate::deps::config::Config;
use crate::util::file;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;

pub struct MapCfgs {
    map: HashMap<String, Rc<MapCfg>>,
}

impl MapCfgs {
    pub fn load() -> MapCfgs {
        let mut map = HashMap::new();

        file::list("config/map/", |p| {
            if p.is_file() {
                fs::read("sjsifjsfi").expect("Failed to read file");
            }
            Ok(())
        })
        .expect("Failed to load config/map/");

        // config::load("config/scene2.txt")
        //     .into_iter()
        //     .map(|i| Rc::new(SceneCfg::new(i)))
        //     .map(|i| map.insert(i.id, i))
        //     .map(|i| i.map(|c| c.id))
        //     .for_each(|i| i.expect_none("Duplicate scene id"));

        MapCfgs { map }
    }

    // pub fn get(&self, id: u32) -> Option<&Rc<MapCfg>> {
    //     self.map.get(&id)
    // }
}

pub struct MapCfg {
    pub id: u32,
}

impl MapCfg {
    fn new<C: Config>(c: C) -> MapCfg {
        let id = c.u32("id").get();

        MapCfg { id }
    }
}
