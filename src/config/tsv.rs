use super::Config;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::iter;
use std::path::Path;

struct Cfg {
    map: HashMap<String, String>,
}

impl Config for Cfg {
    fn get(&self, key: &str) -> Option<&str> {
        self.map.get(key).map(|s| s.as_ref())
    }
}

pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Vec<impl Config>> {
    let s = fs::read_to_string(path)?;
    let v = s.split('\n').fold(Vec::<Vec<&str>>::new(), |mut v, l| {
        v.push(l.split('\t').map(|f| f.trim()).collect());
        v
    });

    let mut r = vec![];
    if v.len() > 2 {
        let keys = &v[1]; // TODO check for duplicates?
        v.iter().skip(2).for_each(|vals| {
            assert!(
                vals.len() <= keys.len(),
                "The number of values must be less than or equal to the number of keys: {}, {}",
                keys.len(),
                vals.len()
            );

            let mut map = HashMap::new();

            keys.iter()
                .copied()
                .zip(vals.iter().copied().chain(iter::repeat("")))
                .for_each(|(key, val)| map.insert(key.to_string(), val.to_string()).unwrap_none());

            r.push(Cfg { map });
        });
    }
    Ok(r)
}
