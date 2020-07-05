use super::Config;
use super::Val;
use std::fmt;
use std::str::FromStr;

pub struct Str<'a, C: Config> {
    cfg: &'a C,
    key: &'a str,
    val: &'a str,
}

impl<'a, C: Config> Str<'a, C> {
    pub fn new(cfg: &'a C, key: &'a str, val: &'a str) -> Str<'a, C> {
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

    pub fn to<T>(self) -> Val<'a, C, T>
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

impl<C: Config> fmt::Display for Str<'_, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The value [{}] of [{}:{}]", self.val, self.cfg, self.key)
    }
}

impl<C: Config> fmt::Debug for Str<'_, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The value [{}] of [{}:{}]", self.val, self.cfg, self.key)
    }
}
