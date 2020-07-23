use std::convert::TryInto;

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
        let l = self.read_u32().try_into().unwrap();
        let r = &self[..l];
        *self = &self[l..];
        r
    }

    fn read_str(&mut self) -> &str {
        let b = self.read_bytes();
        std::str::from_utf8(b).expect("Failed to read string")
    }
}

pub trait BytesMut {
    fn write_u8(&mut self, val: u8);
    fn write_u16(&mut self, val: u16);
    fn write_u32(&mut self, val: u32);
    fn write_bytes(&mut self, val: &[u8]);
    fn write_str(&mut self, val: &str);
}

impl BytesMut for Vec<u8> {
    fn write_u8(&mut self, val: u8) {
        self.push(val);
    }

    fn write_u16(&mut self, val: u16) {
        self.write_u8((val >> 8) as u8);
        self.write_u8(val as u8);
    }

    fn write_u32(&mut self, val: u32) {
        self.write_u16((val >> 16) as u16);
        self.write_u16(val as u16);
    }

    fn write_bytes(&mut self, val: &[u8]) {
        self.write_u32(val.len().try_into().unwrap());
        self.extend(val);
    }

    fn write_str(&mut self, val: &str) {
        self.write_bytes(val.as_bytes());
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
