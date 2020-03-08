#![allow(dead_code)]

pub mod lua52;

pub use lua52::Lua52;

#[derive(Clone)]
pub enum OpArgMode {
    OpArgN, // unused argument
    OpArgU, // used argument
    OpArgR, // argument is register or jump offset
    OpArgK, // argument is const or register const
}
#[derive(Clone)]
pub enum InstMode {
    #[allow(non_camel_case_types)]
    iABC,
    #[allow(non_camel_case_types)]
    iABx,
    #[allow(non_camel_case_types)]
    iAsBx,
    #[allow(non_camel_case_types)]
    iAx,
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

pub trait Target {
    fn datatype(datatype: &str) -> isize;
    fn opcode(opcode: &str) -> (u32, u8, u8, OpArgMode, OpArgMode, InstMode);
}
