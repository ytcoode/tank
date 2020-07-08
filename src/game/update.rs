use crate::game::Game;
use std::time::Instant;

pub trait Update {
    fn update(&mut self, game: &Game, now: Instant);

    fn destroyed(&self) -> bool;
}

pub fn update<U: Update>(v: &mut Vec<U>, game: &Game, now: Instant) {
    for i in (0..v.len()).rev() {
        let e = &mut v[i];
        if e.destroyed() {
            v.swap_remove(i);
        } else {
            e.update(game, now);
        }
    }
}
