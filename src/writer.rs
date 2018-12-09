#![allow(dead_code)]

pub struct Writer {
    vec: Vec<u8>,
    endian: bool, // true for big-endian
}

impl Writer {
    pub fn new() -> Self {
        Writer {
            vec: Vec::new(),
            endian: false,
        }
    }
    pub fn endian(self, endian: bool) -> Self {
        Writer {
            vec: self.vec,
            endian,
        }
    }
    pub fn into_inner(self) -> Vec<u8> {
        self.vec
    }
    pub fn vec_mut(&mut self) -> &mut Vec<u8> {
        &mut self.vec
    }
}

pub trait WriteNumber<T> {
    fn write(&mut self, num: T);
}

impl WriteNumber<u8> for Writer {
    fn write(&mut self, num: u8) {
        self.vec_mut().push(num)
    }
}
impl WriteNumber<u32> for Writer {
    fn write(&mut self, num: u32) {
        self.vec_mut().push((num & 0xFF) as u8);
        self.vec_mut().push(((num & (0xFF << 8)) >> 8) as u8);
        self.vec_mut().push(((num & (0xFF << 16)) >> 16) as u8);
        self.vec_mut().push(((num & (0xFF << 24)) >> 24) as u8);
    }
}
impl WriteNumber<u64> for Writer {
    fn write(&mut self, num: u64) {
        self.vec_mut().push((num & 0xFF) as u8);
        self.vec_mut().push(((num & (0xFF << 8)) >> 8) as u8);
        self.vec_mut().push(((num & (0xFF << 16)) >> 16) as u8);
        self.vec_mut().push(((num & (0xFF << 24)) >> 24) as u8);
        self.vec_mut().push(((num & (0xFF << 32)) >> 32) as u8);
        self.vec_mut().push(((num & (0xFF << 40)) >> 40) as u8);
        self.vec_mut().push(((num & (0xFF << 48)) >> 48) as u8);
        self.vec_mut().push(((num & (0xFF << 56)) >> 56) as u8);
    }
}
impl WriteNumber<f64> for Writer {
    fn write(&mut self, num: f64) {
        self.write(num.to_bits())
    }
}
impl WriteNumber<Vec<u8>> for Writer {
    fn write(&mut self, mut num: Vec<u8>) {
        self.vec_mut().append(&mut num)
    }
}
