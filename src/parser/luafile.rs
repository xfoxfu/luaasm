use super::{Func, Rule};
use crate::lua::{Endian, LuaVersion};
use crate::writer::{WriteObj, Writer};
use pest::iterators::Pair;

#[derive(Debug)]
pub struct LuaFile {
    pub main: Func,
    pub endian: Endian,
    pub lua_version: LuaVersion,
}

impl From<Pair<'_, Rule>> for LuaFile {
    fn from(pest: Pair<'_, Rule>) -> LuaFile {
        LuaFile {
            main: pest.into_inner().next().unwrap().into(),
            endian: Endian::LittleEndian,
            lua_version: LuaVersion::Lua52,
        }
    }
}

impl Into<Vec<u8>> for LuaFile {
    fn into(self) -> Vec<u8> {
        // common header
        let mut writer = Writer::new();
        // Lua bytecode signature
        writer.write(0x1Bu8);
        writer.write(0x4Cu8);
        writer.write(0x75u8);
        writer.write(0x61u8);
        // [u8 version] Version number (0x52 for Lua 5.2, etc)
        match self.lua_version {
            LuaVersion::Lua52 => writer.write(0x52u8),
        }
        // [u8 impl] Implementation (0 for reference impl)
        writer.write(0x00u8);
        // [u8 endian] Big-endian flag
        match &self.endian {
            Endian::LittleEndian => writer.write(0x01u8),
            Endian::BigEndian => writer.write(0x00u8),
        }
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
