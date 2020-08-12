use self::map::Map;
use self::unit::Unit;
use crate::game::bullet::Bullet;
use crate::game::common::view::PlayerView;
use crate::game::tank::Tank;
use ggez::Context;
use std::cell::{Cell, RefCell, RefMut};
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::time::Instant;

pub use cfg::*;

mod cfg;
pub mod map;
pub mod unit;

const PLAYER_TANK_ID: u32 = 1;

pub struct Scene {
    cfg: Rc<SceneCfg>,
    map: RefCell<Map>,
    tanks: RefCell<HashMap<u32, Rc<Tank>>>,
    bullets: RefCell<HashMap<u32, Rc<Bullet>>>,
    pub destroyed_tanks: RefCell<Vec<u32>>,
    pub destroyed_bullets: RefCell<Vec<u32>>,
    view: RefCell<PlayerView>,
    id_counter: Cell<u32>,
}

impl Scene {
    pub fn new(cfg: Rc<SceneCfg>, ctx: &mut Context) -> Rc<Self> {
        let map = RefCell::new(Map::new(cfg.map.clone()));
        let tanks = RefCell::new(HashMap::new());
        let bullets = RefCell::new(HashMap::new());
        let destroyed_tanks = RefCell::new(Vec::new());
        let destroyed_bullets = RefCell::new(Vec::new());
        let view = RefCell::new(PlayerView::new(0, 0, ctx, &cfg.map));
        let id_counter = Cell::new(0);

        let scene = Rc::new(Scene {
            cfg,
            map,
            tanks,
            bullets,
            destroyed_tanks,
            destroyed_bullets,
            view,
            id_counter,
        });

        scene.cfg.tanks.iter().for_each(|(tank_cfg, x, y)| {
            let t = Tank::new(
                scene.next_unit_id(),
                tank_cfg.clone(),
                *x,
                *y,
                scene.clone(),
            );
            scene.add_tank(t)
        });

        scene
    }

    pub fn next_unit_id(&self) -> u32 {
        self.id_counter.update(|x| x + 1)
    }

    fn add_tank(&self, tank: Tank) {
        let tank = Rc::new(tank);
        self.tanks
            .borrow_mut()
            .insert(tank.id(), tank.clone())
            .expect_none("Duplicate tank id");
        self.map.borrow_mut().add(tank);
    }

    fn remove_tank(&self, id: u32) {
        let unit = self.tanks.borrow_mut().remove(&id).unwrap();
        self.map.borrow_mut().remove(unit);
    }

    pub fn add_bullet(&self, bullet: Bullet) {
        let bullet = Rc::new(bullet);
        self.bullets
            .borrow_mut()
            .insert(bullet.id(), bullet.clone())
            .expect_none("Duplicate tank id");
        self.map.borrow_mut().add(bullet);
    }

    fn remove_bullet(&self, id: u32) {
        let unit = self.bullets.borrow_mut().remove(&id).unwrap();
        self.map.borrow_mut().remove(unit);
    }

    pub fn draw(&self, ctx: &mut Context) {
        self.map.borrow_mut().draw(ctx, &self.view.borrow());
    }

    pub fn update(&self, now: Instant) {
        self.remove_destroyed_units();

        // tanks
        let tanks = self.tanks.borrow();
        tanks.values().for_each(|t| t.update(now));

        let t = tanks.get(&PLAYER_TANK_ID).unwrap();
        let (x, y) = t.position();
        self.view.borrow_mut().update(x, y, &self.cfg.map);

        // bullets
        self.bullets.borrow().values().for_each(|b| b.update(now));
    }

    fn remove_destroyed_units(&self) {
        // tanks
        let mut v = self.destroyed_tanks.borrow_mut();
        v.iter().copied().for_each(|id| self.remove_tank(id));
        v.clear();

        // bullets
        let mut v = self.destroyed_bullets.borrow_mut();
        v.iter().copied().for_each(|id| self.remove_bullet(id));
        v.clear();
    }

    pub fn player_tank_move_to(&self, x: u32, y: u32, now: Instant) {
        let v = self.view.borrow();
        let x = v.x + x;
        let y = v.y + y;

        let tanks = self.tanks.borrow();
        let t = tanks.get(&PLAYER_TANK_ID).unwrap();
        t.move_to(x, y, now);
    }

    pub fn player_tank_fire(&self, now: Instant) {
        self.tanks.borrow().get(&PLAYER_TANK_ID).unwrap().fire(now);
    }

    pub fn map(&self) -> RefMut<'_, Map> {
        self.map.borrow_mut()
    }

    pub fn size(&self) -> (u32, u32) {
        let m = &self.cfg.map;
        (m.width, m.height)
    }
}
