use std::fmt;

pub trait Unit: fmt::Debug {
    fn id(&self) -> u32;

    fn x(&self) -> u32;
    fn y(&self) -> u32;

    fn view(&self) -> u32;
}
