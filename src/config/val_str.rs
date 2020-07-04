use super::Config;

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
            self.key,
        );
        self
    }

    pub fn get(self) -> &'a str {
        self.val
    }
}
