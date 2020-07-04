use super::Config;
use super::Str;

pub struct U32<'a> {
    str: Str<'a>,
    val: u32,
}

impl<'a> U32<'a> {
    pub fn new(str: Str<'a>) -> U32<'a> {
        U32 { str, val }
    }

    pub fn get(self) -> u32 {
        self.val
    }
}
