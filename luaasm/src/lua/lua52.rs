#![allow(unused_assignments)]

use super::{InstMode, LuaTarget, OpArgMap, OpArgMask};
use luaasm_derive::{impl_datatype, impl_oparg, impl_opmode};

pub struct Lua52;

impl LuaTarget for Lua52 {
    impl_datatype! { datatype =>
        #define LUA_TNONE          (-1)

        #define LUA_TNIL            0
        #define LUA_TBOOLEAN        1
        #define LUA_TLIGHTUSERDATA  2
        #define LUA_TNUMBER         3
        #define LUA_TSTRING         4
        #define LUA_TTABLE          5
        #define LUA_TFUNCTION       6
        #define LUA_TUSERDATA       7
        #define LUA_TTHREAD         8

        #define LUA_NUMTYPES        9
    }

    impl_opmode! { opcode =>
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

    impl_oparg! { oparg =>
        OP_MOVE     A B     "R(A) := R(B)"
        OP_LOADK    A Bx    "R(A) := Kst(Bx)"
        OP_LOADKX   A       "R(A) := Kst(extra arg)"
        OP_LOADBOOL A B C   "R(A) := (Bool)B; if (C) pc++"
        OP_LOADNIL  A B     "R(A), R(A+1), ..., R(A+B) := nil"
        OP_GETUPVAL A B     "R(A) := UpValue[B]"

        OP_GETTABUP A B C   "R(A) := UpValue[B][RK(C)]"
        OP_GETTABLE A B C   "R(A) := R(B)[RK(C)]"

        OP_SETTABUP A B C   "UpValue[A][RK(B)] := RK(C)"
        OP_SETUPVAL A B     "UpValue[B] := R(A)"
        OP_SETTABLE A B C   "R(A)[RK(B)] := RK(C)"

        OP_NEWTABLE A B C   "R(A) := {} (size = B,C)"

        OP_SELF     A B C   "R(A+1) := R(B); R(A) := R(B)[RK(C)]"

        OP_ADD      A B C   "R(A) := RK(B) + RK(C)"
        OP_SUB      A B C   "R(A) := RK(B) - RK(C)"
        OP_MUL      A B C   "R(A) := RK(B) * RK(C)"
        OP_DIV      A B C   "R(A) := RK(B) / RK(C)"
        OP_MOD      A B C   "R(A) := RK(B) % RK(C)"
        OP_POW      A B C   "R(A) := RK(B) ^ RK(C)"
        OP_UNM      A B     "R(A) := -R(B)"
        OP_NOT      A B     "R(A) := not R(B)"
        OP_LEN      A B     "R(A) := length of R(B)"

        OP_CONCAT   A B C   "R(A) := R(B).. ... ..R(C)"

        OP_JMP      A sBx   "pc+=sBx; if (A) close all upvalues >= R(A - 1)"
        OP_EQ       A B C   "if ((RK(B) == RK(C)) ~= A) then pc++"
        OP_LT       A B C   "if ((RK(B) <  RK(C)) ~= A) then pc++"
        OP_LE       A B C   "if ((RK(B) <= RK(C)) ~= A) then pc++"

        OP_TEST     A C     "if not (R(A) <=> C) then pc++"
        OP_TESTSET  A B C   "if (R(B) <=> C) then R(A) := R(B) else pc++"

        OP_CALL     A B C   "R(A), ... ,R(A+C-2) := R(A)(R(A+1), ... ,R(A+B-1))"
        OP_TAILCALL A B C   "return R(A)(R(A+1), ... ,R(A+B-1))"
        OP_RETURN   A B     "return R(A), ... ,R(A+B-2)	(see note)"

        OP_FORLOOP  A sBx   "R(A)+=R(A+2);\nif R(A) <?= R(A+1) then { pc+=sBx; R(A+3)=R(A) }"
        OP_FORPREP  A sBx   "R(A)-=R(A+2); pc+=sBx"

        OP_TFORCALL A C     "R(A+3), ... ,R(A+2+C) := R(A)(R(A+1), R(A+2));"
        OP_TFORLOOP A sBx   "if R(A+1) ~= nil then { R(A)=R(A+1); pc += sBx }"

        OP_SETLIST  A B C   "R(A)[(C-1)*FPF+i] := R(A+i), 1 <= i <= B"

        OP_CLOSURE  A Bx    "R(A) := closure(KPROTO[Bx])"

        OP_VARARG   A B     "R(A), R(A+1), ..., R(A+B-2) = vararg"

        OP_EXTRAARG Ax      "extra (larger) argument for previous opcode"
    }
}
