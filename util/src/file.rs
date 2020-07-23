use std::fmt::Display;
use std::io;
use std::path::{Path, PathBuf};

pub fn walk<P, F>(p: P, f: &mut F) -> io::Result<()>
where
    P: AsRef<Path>,
    F: FnMut(&Path) -> io::Result<()>,
{
    p.as_ref().read_dir()?.try_for_each(|e| {
        let e = e?;
        let p = e.path();
        if e.file_type()?.is_dir() {
            walk(p, f)
        } else {
            f(p.as_ref())
        }
    })
}

pub fn list<P>(p: P) -> Vec<PathBuf>
where
    P: AsRef<Path> + Display + Copy,
{
    list_try(p).expect(format!("Failed to list files in directory: {}", p).as_ref())
}

pub fn list_try<P>(p: P) -> io::Result<Vec<PathBuf>>
where
    P: AsRef<Path>,
{
    p.as_ref()
        .read_dir()?
        .map(|e| e.map(|e| e.path()))
        .collect()
}

pub fn name<P: AsRef<Path>>(p: P) -> String {
    p.as_ref()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}
