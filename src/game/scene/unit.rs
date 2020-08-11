use crate::game::common::{position::Position, view::PlayerView};
use ggez::Context;
use std::cell::{Cell, Ref};
use std::fmt;
use std::rc::Rc;

pub trait Unit: fmt::Debug + fmt::Display {
    fn id(&self) -> u32;
    fn name(&self) -> &str;
    fn position(&self) -> Ref<'_, Position>;

    fn view(&self) -> Option<&View>;
    fn view_enter(&self, viewer: &dyn Unit);
    fn view_leave(&self, viewer: &dyn Unit);

    fn map_cell(&self) -> &MapCell;
    fn draw(&self, ctx: &mut Context, view: &PlayerView);
}

#[derive(Debug, Default)]
pub struct View {
    pub range: u32,
    i1: Cell<u32>,
    i2: Cell<u32>,
    j1: Cell<u32>,
    j2: Cell<u32>,
}

impl View {
    pub fn new(range: u32) -> Self {
        View {
            range,
            ..Default::default()
        }
    }

    pub fn current_update(&self, i1: u32, i2: u32, j1: u32, j2: u32) {
        self.i1.set(i1);
        self.i2.set(i2);
        self.j1.set(j1);
        self.j2.set(j2);
    }

    pub fn current(&self) -> (u32, u32, u32, u32) {
        (self.i1.get(), self.i2.get(), self.j1.get(), self.j2.get())
    }
}

#[derive(Debug, Default)]
pub struct MapCell {
    cell_idx: Cell<Option<(u32, u32)>>,
}

impl MapCell {
    pub fn exists(&self) -> bool {
        self.cell_idx.get() != None
    }

    pub fn get(&self) -> (u32, u32) {
        self.cell_idx.get().unwrap()
    }

    pub fn set(&self, i: u32, j: u32) {
        self.cell_idx.set(Some((i, j)));
    }

    pub fn clear(&self) {
        self.cell_idx.set(None);
    }
}
