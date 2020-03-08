#![allow(dead_code)]

use std::io::Write;

pub struct Writer {
    vec: Vec<u8>,
    big_endian: bool, // true for big-endian
}

impl Writer {
    pub fn new() -> Self {
        Writer {
            vec: Vec::new(),
            big_endian: false,
        }
    }
    pub fn big_endian(self, big_endian: bool) -> Self {
        Writer {
            vec: self.vec,
            big_endian,
        }
    }
    pub fn into_inner(self) -> Vec<u8> {
        self.vec
    }
    pub fn vec_mut(&mut self) -> &mut Vec<u8> {
        &mut self.vec
    }
    pub fn write_to_file(&self, file: &mut std::fs::File) -> Result<(), std::io::Error> {
        file.write_all(&self.vec)
    }
    pub fn write_u8(&mut self, num: u8) {
        self.vec_mut().push(num)
    }
}

pub trait WriteObj<T> {
    fn write(&mut self, obj: T);
}

impl WriteObj<u8> for Writer {
    fn write(&mut self, num: u8) {
        self.write_u8(num)
    }
}
impl WriteObj<u32> for Writer {
    fn write(&mut self, num: u32) {
        if self.big_endian {
            self.vec_mut().push(((num & (0xFF << 24)) >> 24) as u8);
            self.vec_mut().push(((num & (0xFF << 16)) >> 16) as u8);
            self.vec_mut().push(((num & (0xFF << 8)) >> 8) as u8);
            self.vec_mut().push((num & 0xFF) as u8);
        } else {
            self.vec_mut().push((num & 0xFF) as u8);
            self.vec_mut().push(((num & (0xFF << 8)) >> 8) as u8);
            self.vec_mut().push(((num & (0xFF << 16)) >> 16) as u8);
            self.vec_mut().push(((num & (0xFF << 24)) >> 24) as u8);
        }
    }
}
impl WriteObj<u64> for Writer {
    fn write(&mut self, num: u64) {
        if self.big_endian {
            self.vec_mut().push(((num & (0xFF << 56)) >> 56) as u8);
            self.vec_mut().push(((num & (0xFF << 48)) >> 48) as u8);
            self.vec_mut().push(((num & (0xFF << 40)) >> 40) as u8);
            self.vec_mut().push(((num & (0xFF << 32)) >> 32) as u8);
            self.vec_mut().push(((num & (0xFF << 24)) >> 24) as u8);
            self.vec_mut().push(((num & (0xFF << 16)) >> 16) as u8);
            self.vec_mut().push(((num & (0xFF << 8)) >> 8) as u8);
            self.vec_mut().push((num & 0xFF) as u8);
        } else {
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
}
impl WriteObj<f64> for Writer {
    fn write(&mut self, num: f64) {
        self.write(num.to_bits())
    }
}
impl WriteObj<Vec<u8>> for Writer {
    fn write(&mut self, mut num: Vec<u8>) {
        self.vec_mut().append(&mut num)
    }
}

pub trait WriteTo {
    fn write_to(self, target: &mut Writer);
}

impl<T> WriteTo for T
where
    Writer: WriteObj<T>,
{
    fn write_to(self, target: &mut Writer) {
        target.write(self)
    }
}
