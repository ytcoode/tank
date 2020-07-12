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
        let map = file::list2("config/map/")
            .expect("Failed to list files in directory [config/map/]")
            .into_iter()
            .filter(|p| p.is_file())
            .map(|p| {
                (
                    p.to_str().unwrap().to_string(),
                    Rc::new(MapCfg::new(
                        fs::read(p.as_path())
                            .expect(format!("Failed to read file {:?}", p).as_str()),
                    )),
                )
            })
            .collect();

        MapCfgs { map }
    }

    // pub fn get(&self, id: u32) -> Option<&Rc<MapCfg>> {
    //     self.map.get(&id)
    // }
}

pub struct MapCfg {}

impl MapCfg {
    fn new(v: Vec<u8>) -> MapCfg {
        MapCfg {}
    }
}
