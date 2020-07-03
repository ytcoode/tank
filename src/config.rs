use std::io;
use std::path::Path;

mod tsv;

pub trait Config {
    fn get(&self, key: &str) -> Option<&str>;
}

pub fn load<P: AsRef<Path>>(file: String) -> io::Result<Vec<impl Config>> {
    tsv::load(file)
}
