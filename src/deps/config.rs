use std::fmt;
use std::marker::Sized;
use std::path::Path;

mod cfg_tsv;
mod val;
mod val_str;

pub use val::*;
pub use val_str::*;

pub trait Config: fmt::Display {
    fn str<'a>(&'a self, key: &'a str) -> Str<'a, Self>
    where
        Self: Sized;

    fn u32<'a>(&'a self, key: &'a str) -> Val<Str<'a, Self>, u32>
    where
        Self: Sized,
    {
        self.str(key).not_empty().to::<u32>()
    }

    fn u16<'a>(&'a self, key: &'a str) -> Val<Str<'a, Self>, u16>
    where
        Self: Sized,
    {
        self.str(key).not_empty().to::<u16>()
    }
}

pub fn load<P>(path: P) -> Vec<impl Config>
where
    P: AsRef<Path> + fmt::Display + Copy,
{
    println!("load: {}", path);
    cfg_tsv::load(path).expect(format!("Failed to load config file {}", path).as_str())
}
