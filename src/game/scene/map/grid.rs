use crate::game::common::view::PlayerView;
use crate::game::scene::unit::Unit;
use ggez::Context;
use std::collections::HashMap;
use std::convert::TryInto;
use std::rc::Rc;

pub struct Grid {
    pub rows: u32,
    pub cols: u32,
    cells: Vec<Cell>,
}

impl Grid {
    pub fn new(rows: u32, cols: u32) -> Self {
        let cells = vec![Cell::default(); (rows * cols).try_into().unwrap()];
        Grid { rows, cols, cells }
    }

    pub fn add(&mut self, i: u32, j: u32, unit: Rc<dyn Unit>) {
        assert!(!unit.map_cell().exists());
        unit.map_cell().set(i, j);
        self.cell_mut(i, j).add(unit);
    }

    pub fn remove(&mut self, i: u32, j: u32, id: u32) -> Rc<dyn Unit> {
        let unit = self.cell_mut(i, j).remove(id);
        unit.map_cell().clear();
        unit
    }

    pub fn add_viewer(&mut self, i: u32, j: u32, unit: Rc<dyn Unit>) {
        self.cell_mut(i, j).add_viewer(unit);
    }

    pub fn remove_viewer(&mut self, i: u32, j: u32, id: u32) {
        self.cell_mut(i, j).remove_viewer(id);
    }

    pub fn unit_moved(&mut self, i1: u32, j1: u32, i2: u32, j2: u32, id: u32) {
        let unit = self.cell_mut(i1, j1).remove_silently(id);
        let c1 = self.cell(i1, j1);
        let c2 = self.cell(i2, j2);

        c1.viewers
            .values()
            .filter(|u| u.id() != unit.id() && !c2.viewers.contains_key(&u.id()))
            .for_each(|v| unit.view_leave(v.as_ref()));

        c2.viewers
            .values()
            .filter(|u| u.id() != unit.id() && !c1.viewers.contains_key(&u.id()))
            .for_each(|v| unit.view_enter(v.as_ref()));

        unit.map_cell().set(i2, j2);
        self.cell_mut(i2, j2).add_silently(unit);
    }

    pub fn for_each<F>(&self, i: u32, j: u32, f: F)
    where
        F: FnMut(&Rc<dyn Unit>),
    {
        self.cell(i, j).for_each(f);
    }

    pub fn draw(&self, i: u32, j: u32, ctx: &mut Context, view: &PlayerView) {
        self.cell(i, j).draw(ctx, view);
    }

    fn cell(&self, i: u32, j: u32) -> &Cell {
        &self.cells[self.cell_idx(i, j)]
    }

    fn cell_mut(&mut self, i: u32, j: u32) -> &mut Cell {
        let k = self.cell_idx(i, j);
        &mut self.cells[k]
    }

    fn cell_idx(&self, i: u32, j: u32) -> usize {
        (i * self.rows + j).try_into().unwrap()
    }
}

#[derive(Clone, Default)]
pub struct Cell {
    units: HashMap<u32, Rc<dyn Unit>>,
    viewers: HashMap<u32, Rc<dyn Unit>>,
}

impl Cell {
    fn add(&mut self, unit: Rc<dyn Unit>) {
        self.viewers
            .values()
            .filter(|u| u.id() != unit.id())
            .for_each(|v| unit.view_enter(v.as_ref()));
        self.add_silently(unit);
    }

    fn add_silently(&mut self, unit: Rc<dyn Unit>) {
        self.units.insert(unit.id(), unit).unwrap_none();
    }

    fn remove(&mut self, id: u32) -> Rc<dyn Unit> {
        let unit = self.remove_silently(id);
        self.viewers
            .values()
            .filter(|u| u.id() != unit.id())
            .for_each(|v| unit.view_leave(v.as_ref()));
        unit
    }

    fn remove_silently(&mut self, id: u32) -> Rc<dyn Unit> {
        self.units.remove(&id).unwrap()
    }

    fn add_viewer(&mut self, unit: Rc<dyn Unit>) {
        self.units
            .values()
            .filter(|u| u.id() != unit.id())
            .for_each(|u| u.view_enter(unit.as_ref()));
        self.viewers.insert(unit.id(), unit).unwrap_none();
    }

    fn remove_viewer(&mut self, id: u32) {
        let unit = self.viewers.remove(&id).unwrap();
        self.units
            .values()
            .filter(|u| u.id() != unit.id())
            .for_each(|u| u.view_leave(unit.as_ref()));
    }

    fn for_each<F>(&self, f: F)
    where
        F: FnMut(&Rc<dyn Unit>),
    {
        self.units.values().for_each(f);
    }

    fn draw(&self, ctx: &mut Context, view: &PlayerView) {
        self.units.values().for_each(|u| u.draw(ctx, view));
    }
}
