use std::cell::RefCell;
use std::fmt;
use std::ops::DerefMut;

pub trait Unit: fmt::Debug + fmt::Display {
    fn id(&self) -> u32;
    fn name(&self) -> &str;

    fn x(&self) -> u32;
    fn y(&self) -> u32;

    fn view(&self) -> Option<&View>;
    fn view_enter(&self, viewer: &dyn Unit);
}

#[derive(Debug)]
pub struct View {
    pub range: u32,
    pub last: RefCell<Range>,
}

impl View {
    pub fn new(range: u32) -> Self {
        View {
            range,
            last: Default::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Range {
    pub i1: u32,
    pub i2: u32,
    pub j1: u32,
    pub j2: u32,
}

impl Range {
    pub fn update(&mut self, i1: u32, i2: u32, j1: u32, j2: u32) {
        self.i1 = i1;
        self.i2 = i2;
        self.j1 = j1;
        self.j2 = j2;
    }
}
