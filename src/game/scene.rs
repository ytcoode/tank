use self::map::Map;
use self::unit::Unit;
use crate::game::common::view::View;
use crate::game::tank::Tank;
use ggez::Context;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

pub use cfg::*;

mod cfg;
pub mod map;
pub mod unit;

pub struct Scene {
    cfg: Rc<SceneCfg>,
    map: Map,
    tanks: HashMap<u32, Rc<Tank>>,
    view: View,
    id_counter: u32,
}

impl Scene {
    pub fn new(cfg: Rc<SceneCfg>, ctx: &mut Context) -> Self {
        let map = Map::new(cfg.map.clone());
        let tanks = HashMap::new();
        let view = View::new(0, 0, ctx, &cfg.map);

        let mut scene = Scene {
            cfg: cfg.clone(),
            map,
            tanks,
            view,
            id_counter: 0,
        };

        cfg.tanks.iter().for_each(|(tank_cfg, x, y)| {
            let t = Tank::new(scene.next_unit_id(), tank_cfg.clone(), *x, *y);
            scene.add_tank(t)
        });

        scene
    }

    fn add_tank(&mut self, tank: Tank) {
        let tank = Rc::new(tank);
        self.tanks
            .insert(tank.id(), tank.clone())
            .expect_none("Duplicate tank id");
        self.map.add(tank);
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        self.map.draw(ctx, &self.view);
    }

    pub fn update_view(&mut self, x: u32, y: u32) {
        let map = &self.cfg.map;
        let view = &mut self.view;
        view.update(
            (view.x + x).min(map.width - 1),
            (view.y + y).min(map.height - 1),
            map,
        );
    }

    pub fn next_unit_id(&mut self) -> u32 {
        self.id_counter += 1;
        self.id_counter
    }
}
