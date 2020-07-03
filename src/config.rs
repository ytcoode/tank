use std::path::Path;

mod tsv;

pub trait Config {
    fn get(&self, key: &str) -> Option<&str>;
}

pub fn load<P>(path: P) -> Vec<impl Config>
where
    P: AsRef<Path> + Copy,
{
    let p = path.as_ref();
    println!("load: {:?}", p);
    tsv::load(path).expect(format!("Failed to load config file {:?}", p).as_str())
}
