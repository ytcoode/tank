use std::io;
use std::path::Path;

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

pub fn list<P, F>(p: P, mut f: F) -> io::Result<()>
where
    P: AsRef<Path>,
    F: FnMut(&Path) -> io::Result<()>,
{
    p.as_ref()
        .read_dir()?
        .try_for_each(|e| f(e?.path().as_ref()))
}

pub fn list2<P>(p: P) -> io::Result<Vec<std::path::PathBuf>>
where
    P: AsRef<Path>,
{
    p.as_ref()
        .read_dir()?
        .map(|e| e.map(|e| e.path()))
        .collect()
}
