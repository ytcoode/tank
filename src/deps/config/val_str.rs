use super::Config;
use super::Val;
use std::any;
use std::fmt;
use std::str::FromStr;

pub struct Str<'a, C: Config> {
    cfg: &'a C,
    key: &'a str,
    val: &'a str,
}

impl<C: Config> fmt::Display for Str<'_, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id = self.cfg.str("id").get();
        let id = if id.len() > 0 {
            format!("[id={}]", id)
        } else {
            id.to_string()
        };

        write!(
            f,
            "The value [{}] of the field [{}:{}]{}",
            self.val, self.cfg, self.key, id
        )
    }
}

impl<'a, C: Config> Str<'a, C> {
    pub fn new(cfg: &'a C, key: &'a str, val: &'a str) -> Self {
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

    pub fn to<T>(self) -> Val<Self, T>
    where
        T: FromStr,
        <T as FromStr>::Err: fmt::Debug,
    {
        let val = self.val.parse().expect(
            format!(
                "{} could not be parsed into type [{}]",
                self,
                any::type_name::<T>()
            )
            .as_str(),
        );
        Val::new(self, val)
    }

    pub fn map<T, F>(self, f: F) -> Val<Self, T>
    where
        F: FnOnce(&Self) -> T,
    {
        let val = f(&self);
        Val::new(self, val)
    }

    pub fn get(&self) -> &'a str {
        self.val
    }
}
