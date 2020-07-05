use super::Config;
use super::Str;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

struct Cfg {
    file: String,
    map: HashMap<String, String>,
}

impl fmt::Display for Cfg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.file)
    }
}

impl Config for Cfg {
    fn str<'a>(&'a self, key: &'a str) -> Str<'a, Cfg> {
        Str::new(
            self,
            key,
            self.map.get(key).map(|s| s.as_str()).unwrap_or(""),
        )
    }
}

pub fn load<P>(path: P) -> io::Result<Vec<impl Config>>
where
    P: AsRef<Path> + fmt::Display + Copy,
{
    let mut lines = Vec::<Vec<&str>>::new();
    let s = fs::read_to_string(path)?;

    s.split('\n')
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .for_each(|l| lines.push(l.split('\t').map(|f| f.trim()).collect()));

    if lines.len() <= 2 {
        return Ok(vec![]);
    }

    let keys = &lines[1];
    keys.iter().zip(1..).for_each(|(k, c)| {
        assert!(
            k.len() > 0,
            "The key must not be empty: {}:{}:{}",
            path,
            2,
            c
        );
    });

    let mut cfgs = Vec::with_capacity(lines.len() - 2);
    lines.iter().skip(2).zip(3..).for_each(|(vals, line)| {
        assert!(
            vals.len() <= keys.len(),
            "The number of values must be less than or equal to the number of keys: {}, {}",
            keys.len(),
            vals.len()
        );

        let mut map = HashMap::new();

        keys.iter()
            .zip(vals.iter()) // allow duplicate keys?
            .for_each(|(key, val)| map.insert(key.to_string(), val.to_string()).unwrap_none());

        cfgs.push(Cfg {
            file: format!("{}:{}", path, line),
            map,
        });
    });

    Ok(cfgs)
}
