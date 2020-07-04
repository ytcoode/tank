use std::fmt;
use std::marker::Sized;
use std::path::Path;

mod cfg_tsv;
mod key;
mod val_str;
mod val_u32;

pub use key::*;
pub use val_str::*;
pub use val_u32::*;

pub trait Config: fmt::Display {
    fn key<'a>(&'a self, key: &'a str) -> Key<'a>
    where
        Self: Sized,
    {
        Key::new(self, key)
    }

    fn str<'a>(&'a self, key: &'a str) -> Str<'a>;

    fn u32(&self) {
        self.key("a");
    }

    // fn get_and_parse<T>(&self, key: &str) -> T
    // where
    //     T: FromStr,
    //     <T as FromStr>::Err: fmt::Debug,
    // {
    //     let s = self.get(key).unwrap_or("");
    //     s.parse::<T>()
    //         .expect(format!("Failed to parse {} as type {}", s, "aa").as_str())
    // }
}

pub fn load<P>(path: P) -> Vec<impl Config>
where
    P: AsRef<Path> + Copy + fmt::Display,
{
    println!("load: {}", path);
    cfg_tsv::load(path).expect(format!("Failed to load config file {}", path).as_str())
}
