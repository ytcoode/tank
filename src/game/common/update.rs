use crate::game::Game;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

pub trait Update {
    fn update(&mut self, game: &Game, now: Instant);

    fn destroyed(&self) -> bool;
}

pub fn update<U: Update>(game: &mut Game, now: Instant) {

    // for i in (0..v.len()).rev() {
    //     let e = &v[i];
    //     if e.borrow().destroyed() {
    //         v.swap_remove(i);
    //     } else {
    //         e.borrow_mut().update(game, now);
    //     }
    // }
}
