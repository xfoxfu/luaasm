#![allow(dead_code)]

pub mod lua52;

pub use lua52::Lua52;

#[derive(Clone)]
pub enum OpArgMask {
    OpArgN, /* argument is not used */
    OpArgU, /* argument is used */
    OpArgR, /* argument is a register or a jump offset */
    OpArgK, /* argument is a constant or register/constant */
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub enum InstMode {
    iABC,
    iABx,
    iAsBx,
    iAx,
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub enum OpArgMap {
    A,
    As,
    Ax,
    B,
    Bx,
    sBx,
    C,
    N,
}

#[derive(Debug)]
pub enum LuaVersion {
    Lua52,
}

#[derive(Debug)]
pub enum Endian {
    BigEndian,
    LittleEndian,
}

pub trait LuaTarget {
    fn datatype(datatype: &str) -> isize;
    fn opcode(opcode: &str) -> (u32, u8, u8, OpArgMask, OpArgMask, InstMode);
    fn oparg(opcode: &str) -> (OpArgMap, OpArgMap, OpArgMap, &str);
}
