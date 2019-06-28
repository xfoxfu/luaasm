use super::{InstMode, OpArgMode};
use lazy_static::lazy_static;
use std::collections::HashMap;

macro_rules! lua_datatype {
  ($($name:ident: $id:literal,)*) => (
    let mut m = HashMap::new();
    $(m.insert(stringify!($name), $id);)*
    m
  )
}

lazy_static! {
    pub static ref LUA_DATATYPE: HashMap<&'static str, u8> = {
        lua_datatype!{
            LUA_TNIL: 0,
            LUA_TBOOLEAN: 1,
            LUA_TLIGHTUSERDATA: 2,
            LUA_TNUMBER: 3,
            LUA_TSTRING: 4,
            LUA_TTABLE: 5,
            LUA_TFUNCTION: 6,
            LUA_TUSERDATA: 7,
            LUA_TTHREAD: 8,
            LUA_NUMTAGS: 9,
        }
    };
}

macro_rules! lua_opcode {
    // (1, 0, 1, OpArgMode::OpArgK, OpArgMode::OpArgN, InstMode::iABx)
  ($($op:ident: opmode($flag1:literal, $flag2:literal, $arg1:ident, $arg2:ident, $inst:ident),)*) => (
    let mut m = HashMap::new();
    $(m.insert(stringify!($op), (0, $flag1, $flag2, OpArgMode::$arg1, OpArgMode::$arg2, InstMode::$inst));)*
    m
  )
}

lazy_static! {
    pub static ref LUA_OPCODE: HashMap<&'static str, (u32, u8, u8, OpArgMode, OpArgMode, InstMode)> = {
        lua_opcode!{
            OP_MOVE: opmode(0, 1, OpArgR, OpArgN, iABC),
            OP_LOADK: opmode(0, 1, OpArgK, OpArgN, iABx),
            OP_LOADKX: opmode(0, 1, OpArgN, OpArgN, iABx),
            OP_LOADBOOL: opmode(0, 1, OpArgU, OpArgU, iABC),
            OP_LOADNIL: opmode(0, 1, OpArgU, OpArgN, iABC),
            OP_GETUPVAL: opmode(0, 1, OpArgU, OpArgN, iABC),
            OP_GETTABUP: opmode(0, 1, OpArgU, OpArgK, iABC),
            OP_GETTABLE: opmode(0, 1, OpArgR, OpArgK, iABC),
            OP_SETTABUP: opmode(0, 0, OpArgK, OpArgK, iABC),
            OP_SETUPVAL: opmode(0, 0, OpArgU, OpArgN, iABC),
            OP_SETTABLE: opmode(0, 0, OpArgK, OpArgK, iABC),
            OP_NEWTABLE: opmode(0, 1, OpArgU, OpArgU, iABC),
            OP_SELF: opmode(0, 1, OpArgR, OpArgK, iABC),
            OP_ADD: opmode(0, 1, OpArgK, OpArgK, iABC),
            OP_SUB: opmode(0, 1, OpArgK, OpArgK, iABC),
            OP_MUL: opmode(0, 1, OpArgK, OpArgK, iABC),
            OP_DIV: opmode(0, 1, OpArgK, OpArgK, iABC),
            OP_MOD: opmode(0, 1, OpArgK, OpArgK, iABC),
            OP_POW: opmode(0, 1, OpArgK, OpArgK, iABC),
            OP_UNM: opmode(0, 1, OpArgR, OpArgN, iABC),
            OP_NOT: opmode(0, 1, OpArgR, OpArgN, iABC),
            OP_LEN: opmode(0, 1, OpArgR, OpArgN, iABC),
            OP_CONCAT: opmode(0, 1, OpArgR, OpArgR, iABC),
            OP_JMP: opmode(0, 0, OpArgR, OpArgN, iAsBx),
            OP_EQ: opmode(1, 0, OpArgK, OpArgK, iABC),
            OP_LT: opmode(1, 0, OpArgK, OpArgK, iABC),
            OP_LE: opmode(1, 0, OpArgK, OpArgK, iABC),
            OP_TEST: opmode(1, 0, OpArgN, OpArgU, iABC),
            OP_TESTSET: opmode(1, 1, OpArgR, OpArgU, iABC),
            OP_CALL: opmode(0, 1, OpArgU, OpArgU, iABC),
            OP_TAILCALL: opmode(0, 1, OpArgU, OpArgU, iABC),
            OP_RETURN: opmode(0, 0, OpArgU, OpArgN, iABC),
            OP_FORLOOP: opmode(0, 1, OpArgR, OpArgN, iAsBx),
            OP_FORPREP: opmode(0, 1, OpArgR, OpArgN, iAsBx),
            OP_TFORCALL: opmode(0, 0, OpArgN, OpArgU, iABC),
            OP_TFORLOOP: opmode(0, 1, OpArgR, OpArgN, iAsBx),
            OP_SETLIST: opmode(0, 0, OpArgU, OpArgU, iABC),
            OP_CLOSURE: opmode(0, 1, OpArgU, OpArgN, iABx),
            OP_VARARG: opmode(0, 1, OpArgU, OpArgN, iABC),
            OP_EXTRAARG: opmode(0, 0, OpArgU, OpArgU, iAx),
        }
    };
}
