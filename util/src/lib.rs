mod ggez2;
pub use ggez2::*;

pub fn div_ceil(a: u32, b: u32) -> u32 {
    a + b - 1 / b
}

pub fn substract(a: u32, b: u32) -> f32 {
    if a > b {
        (a - b) as f32
    } else {
        -((b - a) as f32)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
