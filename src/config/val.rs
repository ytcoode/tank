use std::borrow::Borrow;
use std::cmp::PartialOrd;
use std::fmt;
use std::fmt::Display;

pub struct Val<K: Display, T> {
    key: K,
    val: T,
}

impl<K: Display, T> Display for Val<K, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.key)
    }
}

impl<K: Display, T> Val<K, T> {
    pub fn new(key: K, val: T) -> Self {
        Val { key, val }
    }

    pub fn get(self) -> T {
        self.val
    }
}

impl<K: Display, T: PartialOrd + Display> Val<K, T> {
    pub fn ge<B: Borrow<T>>(self, b: B) -> Self {
        let b = b.borrow();
        assert!(
            self.val.ge(b),
            "{} must be greater than or equal to {}",
            self.key,
            b
        );
        self
    }

    pub fn le<B: Borrow<T>>(self, b: B) -> Self {
        let b = b.borrow();
        assert!(
            self.val.le(b),
            "{} must be less than or equal to {}",
            self.key,
            b
        );
        self
    }

    pub fn range<B: Borrow<T>>(self, min: B, max: B) -> Self {
        let min = min.borrow();
        let max = max.borrow();
        assert!(
            self.val.ge(min) && self.val.le(max),
            "{} must be in the range {} to {} inclusive",
            self.key,
            min,
            max
        );
        self
    }
}
