pub trait Bytes {
    fn read_u8(&mut self) -> u8;
    fn read_u16(&mut self) -> u16;
    fn read_u32(&mut self) -> u32;
    fn read_bytes(&mut self) -> &[u8];
    fn read_str(&mut self) -> &str;
}

impl Bytes for &[u8] {
    fn read_u8(&mut self) -> u8 {
        let r = self[0];
        *self = &self[1..];
        r
    }

    fn read_u16(&mut self) -> u16 {
        (self.read_u8() as u16) << 8 | self.read_u8() as u16
    }

    fn read_u32(&mut self) -> u32 {
        (self.read_u16() as u32) << 16 | self.read_u16() as u32
    }

    fn read_bytes(&mut self) -> &[u8] {
        let l = self.read_u16().into();
        let r = &self[..l];
        *self = &self[l..];
        r
    }

    fn read_str(&mut self) -> &str {
        let b = self.read_bytes();
        std::str::from_utf8(b).expect("Failed to read string")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_u8() {
        let a = vec![1u8, 2, 3];
        let mut a = a.as_slice();

        let v = a.read_u8();
        assert_eq!(v, 1);

        assert_eq!(a, &[2, 3]);
    }
}