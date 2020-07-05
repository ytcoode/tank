use super::Config;
use super::Str;
use std::any;
use std::borrow::Borrow;
use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::str::FromStr;

pub struct Val<K: Debug, T> {
    key: K,
    val: T,
}

impl<K: Debug, T> Val<K, T> {
    pub fn get(self) -> T {
        self.val
    }
}

impl<K: Debug, T> Val<K, T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    pub fn new<C: Config>(str: Str<'_, C>) -> Val<Str<'_, C>, T> {
        let val = str.get().parse().expect(
            format!(
                "{:?} could not be parsed into type [{}]",
                str,
                any::type_name::<T>()
            )
            .as_str(),
        );
        Val { key: str, val }
    }
}

impl<K: Debug, T: PartialOrd + Debug> Val<K, T> {
    pub fn ge<B: Borrow<T>>(self, b: B) -> Self {
        let b = b.borrow();
        assert!(
            self.val.ge(b),
            "{:?} must be greater than or equal to {:?}",
            self.key,
            b
        );
        self
    }

    pub fn le<B: Borrow<T>>(self, b: B) -> Self {
        let b = b.borrow();
        assert!(
            self.val.le(b),
            "{:?} must be less than or equal to {:?}",
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
            "{:?} must be in the range {:?} to {:?} inclusive",
            self.key,
            min,
            max
        );
        self
    }
}
