#![allow(dead_code)]

pub mod lua52;

#[derive(Clone)]
pub enum OpArgMode {
    OpArgN, // 参数未被使用
    OpArgU, // 已使用参数
    OpArgR, // 参数是寄存器或跳转偏移
    OpArgK, // 参数是常量或寄存器常量
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
