use std::fmt;
use std::path::Path;
use std::str::FromStr;

mod tsv;

pub trait Config: fmt::Display {
    fn str(&self, key: &str) -> &str;

    fn u32(&self) {}

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
    tsv::load(path).expect(format!("Failed to load config file {}", path).as_str())
}
