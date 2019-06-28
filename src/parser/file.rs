#![allow(dead_code)]

use super::{func_decl, AstCheck, Func};
use crate::writer::{WriteObj, Writer};
use nom::{call, named, types::CompleteStr};

#[derive(Serialize, Debug, PartialEq)]
pub struct File {
    pub main: Func,
}

named!(
    pub parse_file(CompleteStr) -> File,
    map!(func_decl, |f| File { main: f })
);

impl AstCheck for File {
    fn check(&self) -> Result<(), String> {
        self.main.check()
    }
}

impl Into<Vec<u8>> for File {
    fn into(self) -> Vec<u8> {
        // common header
        let mut writer = Writer::new();
        // Lua bytecode signature
        writer.write(0x1Bu8);
        writer.write(0x4Cu8);
        writer.write(0x75u8);
        writer.write(0x61u8);
        // [u8 version] Version number (0x52 for Lua 5.2, etc)
        writer.write(0x52u8); // TODO: support other version
                              // [u8 impl] Implementation (0 for reference impl)
        writer.write(0x00u8);
        // [u8 endian] Big-endian flag
        writer.write(0x01u8); // TODO: support big-endian
                              // [u8 intsize] Size of integers (usually 4)
        writer.write(0x04u8);
        // [u8 size_t] Size of pointers
        writer.write(0x04u8);
        // [u8 instsize] Size of instructions (always 4)
        writer.write(0x04u8);
        //  [u8 numsize] Size of Lua numbers (usually 8)
        writer.write(0x08u8);
        // [u8 use_int] Use integers instead of floats (usually for embedded)
        writer.write(0x00u8);
        // Lua magic (used to detect presence of EOL conversion)
        writer.write(0x19u8);
        writer.write(0x93u8);
        writer.write(0x0Du8);
        writer.write(0x0Au8);
        writer.write(0x1Au8);
        writer.write(0x0Au8);
        let main: Vec<u8> = self.main.into();
        writer.write(main);

        writer.into_inner()
    }
}
