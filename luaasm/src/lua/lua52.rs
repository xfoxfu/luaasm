#![allow(unused_assignments)]

use super::{InstMode, OpArgMode, Target};
use luaasm_derive::{impl_datatype, impl_opcode};

pub struct Lua52;

impl Target for Lua52 {
    impl_datatype! { datatype =>
        #define LUA_TNONE		(-1)

        #define LUA_TNIL		0
        #define LUA_TBOOLEAN		1
        #define LUA_TLIGHTUSERDATA	2
        #define LUA_TNUMBER		3
        #define LUA_TSTRING		4
        #define LUA_TTABLE		5
        #define LUA_TFUNCTION		6
        #define LUA_TUSERDATA		7
        #define LUA_TTHREAD		8

        #define LUA_NUMTYPES		9
    }

    impl_opcode! { opcode =>
    /*
    ** $Id: lopcodes.c,v 1.49 2012/05/14 13:34:18 roberto Exp $
    ** Opcodes for Lua virtual machine
    ** See Copyright Notice in lua.h
    */
    /*       T  A    B       C     mode       opcode	*/
      opmode(0, 1, OpArgR, OpArgN, iABC)     OP_MOVE
     ,opmode(0, 1, OpArgK, OpArgN, iABx)     OP_LOADK
     ,opmode(0, 1, OpArgN, OpArgN, iABx)     OP_LOADKX
     ,opmode(0, 1, OpArgU, OpArgU, iABC)     OP_LOADBOOL
     ,opmode(0, 1, OpArgU, OpArgN, iABC)     OP_LOADNIL
     ,opmode(0, 1, OpArgU, OpArgN, iABC)     OP_GETUPVAL
     ,opmode(0, 1, OpArgU, OpArgK, iABC)     OP_GETTABUP
     ,opmode(0, 1, OpArgR, OpArgK, iABC)     OP_GETTABLE
     ,opmode(0, 0, OpArgK, OpArgK, iABC)     OP_SETTABUP
     ,opmode(0, 0, OpArgU, OpArgN, iABC)     OP_SETUPVAL
     ,opmode(0, 0, OpArgK, OpArgK, iABC)     OP_SETTABLE
     ,opmode(0, 1, OpArgU, OpArgU, iABC)     OP_NEWTABLE
     ,opmode(0, 1, OpArgR, OpArgK, iABC)     OP_SELF
     ,opmode(0, 1, OpArgK, OpArgK, iABC)     OP_ADD
     ,opmode(0, 1, OpArgK, OpArgK, iABC)     OP_SUB
     ,opmode(0, 1, OpArgK, OpArgK, iABC)     OP_MUL
     ,opmode(0, 1, OpArgK, OpArgK, iABC)     OP_DIV
     ,opmode(0, 1, OpArgK, OpArgK, iABC)     OP_MOD
     ,opmode(0, 1, OpArgK, OpArgK, iABC)     OP_POW
     ,opmode(0, 1, OpArgR, OpArgN, iABC)     OP_UNM
     ,opmode(0, 1, OpArgR, OpArgN, iABC)     OP_NOT
     ,opmode(0, 1, OpArgR, OpArgN, iABC)     OP_LEN
     ,opmode(0, 1, OpArgR, OpArgR, iABC)     OP_CONCAT
     ,opmode(0, 0, OpArgR, OpArgN, iAsBx)    OP_JMP
     ,opmode(1, 0, OpArgK, OpArgK, iABC)     OP_EQ
     ,opmode(1, 0, OpArgK, OpArgK, iABC)     OP_LT
     ,opmode(1, 0, OpArgK, OpArgK, iABC)     OP_LE
     ,opmode(1, 0, OpArgN, OpArgU, iABC)     OP_TEST
     ,opmode(1, 1, OpArgR, OpArgU, iABC)     OP_TESTSET
     ,opmode(0, 1, OpArgU, OpArgU, iABC)     OP_CALL
     ,opmode(0, 1, OpArgU, OpArgU, iABC)     OP_TAILCALL
     ,opmode(0, 0, OpArgU, OpArgN, iABC)     OP_RETURN
     ,opmode(0, 1, OpArgR, OpArgN, iAsBx)    OP_FORLOOP
     ,opmode(0, 1, OpArgR, OpArgN, iAsBx)    OP_FORPREP
     ,opmode(0, 0, OpArgN, OpArgU, iABC)     OP_TFORCALL
     ,opmode(0, 1, OpArgR, OpArgN, iAsBx)    OP_TFORLOOP
     ,opmode(0, 0, OpArgU, OpArgU, iABC)     OP_SETLIST
     ,opmode(0, 1, OpArgU, OpArgN, iABx)     OP_CLOSURE
     ,opmode(0, 1, OpArgU, OpArgN, iABC)     OP_VARARG
     ,opmode(0, 0, OpArgU, OpArgU, iAx)      OP_EXTRAARG
    }
}
