use std::io;
use std::path::Path;

mod tsv;

pub trait Config {
    fn get(&self, key: &str) -> Option<&str>;
}

pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Vec<impl Config>> {
    tsv::load(path)
}
