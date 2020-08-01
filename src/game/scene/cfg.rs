use super::map::{MapCfg, MapCfgs};
use crate::game::tank::{TankCfg, TankCfgs};
use config::{self, Config};
use ggez::Context;
use std::collections::HashMap;
use std::rc::Rc;

pub struct SceneCfgs {
    map: HashMap<u32, Rc<SceneCfg>>,
}

impl SceneCfgs {
    pub fn load(ctx: &mut Context, tank_cfgs: &TankCfgs) -> SceneCfgs {
        let map_cfgs = MapCfgs::load(ctx);
        let mut map = HashMap::new();

        config::load("assets/config/scene.txt")
            .into_iter()
            .map(|c| Rc::new(SceneCfg::new(c, &map_cfgs, tank_cfgs)))
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
    pub tanks: Vec<(Rc<TankCfg>, u32, u32)>,
}

impl SceneCfg {
    fn new<C: Config>(c: C, map_cfgs: &MapCfgs, tank_cfgs: &TankCfgs) -> SceneCfg {
        let id = c.u32("id").get();
        let map = c
            .str("map")
            .not_empty()
            .map(|s| {
                map_cfgs
                    .get(s.get())
                    .expect(format!("{} not found", s).as_str())
                    .clone()
            })
            .get();

        // x=y=tankid_x=y=tankid ...
        let tanks = c
            .str("tanks")
            .not_empty()
            .map(|s| {
                s.get()
                    .split("_")
                    .map(|s| {
                        let a: Vec<u32> = s
                            .split("=")
                            .map(|s| {
                                s.parse().expect(
                                    format!(
                                        "Failed to parse {}: invalid digit found in string!",
                                        s
                                    )
                                    .as_str(),
                                )
                            })
                            .collect();

                        assert!(a.len() == 3, "Failed to parse {}: invalid format!", s);

                        let tank_id = a[0];
                        let tank_cfg = tank_cfgs.get(tank_id).expect(
                            format!("Failed to parse {}: invalid tank id: {}!", s, tank_id)
                                .as_str(),
                        );

                        let x = a[1];
                        let y = a[2];

                        assert!(
                            map.is_walkable(x, y),
                            "Failed to parse {}: tank's position is not walkable: {}-{}!",
                            s,
                            x,
                            y
                        );

                        (tank_cfg.clone(), x, y)
                    })
                    .collect()
            })
            .get();

        SceneCfg { id, map, tanks }
    }
}
