use std::fmt;

pub trait Unit: fmt::Debug {
    fn id(&self) -> u32;

    fn x(&self) -> u32;
    fn y(&self) -> u32;

    fn view(&mut self) -> Option<&mut View>;
}

pub struct View {
    pub range: u32,
    i1: u32,
    i2: u32,
    j1: u32,
    j2: u32,
}
