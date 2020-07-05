use super::Config;
use super::Val;
use std::fmt;
use std::str::FromStr;

pub struct Str<'a> {
    cfg: &'a dyn Config,
    key: &'a str,
    val: &'a str,
}

impl<'a> Str<'a> {
    pub fn new(cfg: &'a dyn Config, key: &'a str, val: &'a str) -> Str<'a> {
        Str { cfg, key, val }
    }

    pub fn not_empty(self) -> Self {
        assert!(
            self.val.len() > 0,
            "The value of [{}:{}] must not be empty!",
            self.cfg,
            self.key
        );
        self
    }

    pub fn to<T>(self) -> Val<'a, T>
    where
        T: FromStr,
        <T as FromStr>::Err: fmt::Debug,
    {
        Val::new(self)
    }

    pub fn get(&self) -> &'a str {
        self.val
    }
}

impl fmt::Display for Str<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The value [{}] of [{}:{}]", self.val, self.cfg, self.key)
    }
}
