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
        lua_datatype! {
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
    let mut counter = 0;
    $(
      m.insert(stringify!($op), (counter, $flag1, $flag2, OpArgMode::$arg1, OpArgMode::$arg2, InstMode::$inst));
      counter+=1;
    )*
    m
  )
}

lazy_static! {
    pub static ref LUA_OPCODE: HashMap<&'static str, (u32, u8, u8, OpArgMode, OpArgMode, InstMode)> = {
        lua_opcode! {
            MOVE: opmode(0, 1, OpArgR, OpArgN, iABC),
            LOADK: opmode(0, 1, OpArgK, OpArgN, iABx),
            LOADKX: opmode(0, 1, OpArgN, OpArgN, iABx),
            LOADBOOL: opmode(0, 1, OpArgU, OpArgU, iABC),
            LOADNIL: opmode(0, 1, OpArgU, OpArgN, iABC),
            GETUPVAL: opmode(0, 1, OpArgU, OpArgN, iABC),
            GETTABUP: opmode(0, 1, OpArgU, OpArgK, iABC),
            GETTABLE: opmode(0, 1, OpArgR, OpArgK, iABC),
            SETTABUP: opmode(0, 0, OpArgK, OpArgK, iABC),
            SETUPVAL: opmode(0, 0, OpArgU, OpArgN, iABC),
            SETTABLE: opmode(0, 0, OpArgK, OpArgK, iABC),
            NEWTABLE: opmode(0, 1, OpArgU, OpArgU, iABC),
            SELF: opmode(0, 1, OpArgR, OpArgK, iABC),
            ADD: opmode(0, 1, OpArgK, OpArgK, iABC),
            SUB: opmode(0, 1, OpArgK, OpArgK, iABC),
            MUL: opmode(0, 1, OpArgK, OpArgK, iABC),
            DIV: opmode(0, 1, OpArgK, OpArgK, iABC),
            MOD: opmode(0, 1, OpArgK, OpArgK, iABC),
            POW: opmode(0, 1, OpArgK, OpArgK, iABC),
            UNM: opmode(0, 1, OpArgR, OpArgN, iABC),
            NOT: opmode(0, 1, OpArgR, OpArgN, iABC),
            LEN: opmode(0, 1, OpArgR, OpArgN, iABC),
            CONCAT: opmode(0, 1, OpArgR, OpArgR, iABC),
            JMP: opmode(0, 0, OpArgR, OpArgN, iAsBx),
            EQ: opmode(1, 0, OpArgK, OpArgK, iABC),
            LT: opmode(1, 0, OpArgK, OpArgK, iABC),
            LE: opmode(1, 0, OpArgK, OpArgK, iABC),
            TEST: opmode(1, 0, OpArgN, OpArgU, iABC),
            TESTSET: opmode(1, 1, OpArgR, OpArgU, iABC),
            CALL: opmode(0, 1, OpArgU, OpArgU, iABC),
            TAILCALL: opmode(0, 1, OpArgU, OpArgU, iABC),
            RETURN: opmode(0, 0, OpArgU, OpArgN, iABC),
            FORLOOP: opmode(0, 1, OpArgR, OpArgN, iAsBx),
            FORPREP: opmode(0, 1, OpArgR, OpArgN, iAsBx),
            TFORCALL: opmode(0, 0, OpArgN, OpArgU, iABC),
            TFORLOOP: opmode(0, 1, OpArgR, OpArgN, iAsBx),
            SETLIST: opmode(0, 0, OpArgU, OpArgU, iABC),
            CLOSURE: opmode(0, 1, OpArgU, OpArgN, iABx),
            VARARG: opmode(0, 1, OpArgU, OpArgN, iABC),
            EXTRAARG: opmode(0, 0, OpArgU, OpArgU, iAx),
        }
    };
}
