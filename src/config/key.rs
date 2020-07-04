use super::Config;
use std::fmt;

pub struct Key<'a> {
    cfg: &'a dyn Config,
    key: &'a str,
}

impl<'a> Key<'a> {
    pub fn new(cfg: &'a dyn Config, key: &'a str) -> Key<'a> {
        Key { cfg, key }
    }
}

impl fmt::Display for Key<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}:{}]", self.cfg, self.key)
    }
}
