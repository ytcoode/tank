use std::cell::RefCell;
use std::fmt;
use std::ops::DerefMut;

pub trait Unit: fmt::Debug {
    fn id(&self) -> u32;

    fn x(&self) -> u32;
    fn y(&self) -> u32;

    fn view(&self) -> Option<&View>;
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
