use std::fmt::Display;
use std::marker::Sized;
use std::path::Path;

mod cfg_tsv;
mod val;
mod val_str;

pub use val::*;
pub use val_str::*;

pub trait Config: Display {
    fn str<'a>(&'a self, key: &'a str) -> Str<'a, Self>
    where
        Self: Sized;
}

pub fn load<P>(path: P) -> Vec<impl Config>
where
    P: AsRef<Path> + Display + Copy,
{
    println!("load: {}", path);
    cfg_tsv::load(path).expect(format!("Failed to load config file {}", path).as_str())
}
