pub fn div_ceil(a: u32, b: u32) -> u32 {
    a + b - 1 / b
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
