use self::map::Map;
use self::unit::Unit;
use crate::game::common::view::PlayerView;
use crate::game::tank::Tank;
use ggez::Context;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::time::Instant;

pub use cfg::*;

mod cfg;
pub mod map;
pub mod unit;

pub struct Scene {
    cfg: Rc<SceneCfg>,
    map: Map,
    tanks: HashMap<u32, Rc<Tank>>,
    view: PlayerView,
    id_counter: u32,
}

impl Scene {
    pub fn new(cfg: Rc<SceneCfg>, ctx: &mut Context) -> Self {
        let map = Map::new(cfg.map.clone());
        let tanks = HashMap::new();
        let view = PlayerView::new(0, 0, ctx, &cfg.map);

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

    fn next_unit_id(&mut self) -> u32 {
        self.id_counter += 1;
        self.id_counter
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

    pub fn update(&mut self, now: Instant) {
        self.tanks.values().for_each(|t| t.update(now));

        let (x, y) = {
            let p = self.player_tank().position();
            (p.x(), p.y())
        };

        // let p = self.player_tank().position();
        // let x = p.x();
        // let y = p.y();

        self.view.update(x, y, &self.cfg.map);
    }

    pub fn player_tank_move_to(&self, x: u32, y: u32, now: Instant) {
        let x = self.view.x + x;
        let y = self.view.y + y;
        self.player_tank().move_to(x, y, now);
    }

    fn player_tank(&self) -> &Rc<Tank> {
        self.tanks.get(&1).unwrap()
    }
}
