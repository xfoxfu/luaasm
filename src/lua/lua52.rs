use super::{InstMode, OpArgMode};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref LUA_DATATYPE: HashMap<&'static str, u8> = {
        let mut m = HashMap::new();
        m.insert("LUA_TNIL", 0);
        m.insert("LUA_TBOOLEAN", 1);
        m.insert("LUA_TLIGHTUSERDATA", 2);
        m.insert("LUA_TNUMBER", 3);
        m.insert("LUA_TSTRING", 4);
        m.insert("LUA_TTABLE", 5);
        m.insert("LUA_TFUNCTION", 6);
        m.insert("LUA_TUSERDATA", 7);
        m.insert("LUA_TTHREAD", 8);
        m.insert("LUA_NUMTAGS", 9);
        m
    };
}

lazy_static! {
    pub static ref LUA_OPCODE: HashMap<&'static str, (u32, u8, u8, OpArgMode, OpArgMode, InstMode)> = {
        let mut m = HashMap::new();
        m.insert(
            "MOVE",
            //	A B	R(A) := R(B)
            (0, 0, 1, OpArgMode::OpArgR, OpArgMode::OpArgN, InstMode::iABC),
        );
        m.insert(
            "LOADK",
            //	A Bx	R(A) := Kst(Bx)
            (1, 0, 1, OpArgMode::OpArgK, OpArgMode::OpArgN, InstMode::iABx),
        );
        m.insert(
            "LOADKX",
            //	A 	R(A) := Kst(extra arg)
            (2, 0, 1, OpArgMode::OpArgN, OpArgMode::OpArgN, InstMode::iABx),
        );
        m.insert(
            "LOADBOOL",
            //	A B C	R(A) := (Bool)B; if (C) pc++
            (3, 0, 1, OpArgMode::OpArgU, OpArgMode::OpArgU, InstMode::iABC),
        );
        m.insert(
            "LOADNIL",
            //	A B	R(A), R(A+1), ..., R(A+B) := nil
            (4, 0, 1, OpArgMode::OpArgU, OpArgMode::OpArgN, InstMode::iABC),
        );
        m.insert(
            "GETUPVAL",
            //	A B	R(A) := UpValue[B]
            (5, 0, 1, OpArgMode::OpArgU, OpArgMode::OpArgN, InstMode::iABC),
        );
        m.insert(
            "GETTABUP",
            //	A B C	R(A) := UpValue[B][RK(C)]
            (6, 0, 1, OpArgMode::OpArgU, OpArgMode::OpArgK, InstMode::iABC),
        );
        m.insert(
            "GETTABLE",
            //	A B C	R(A) := R(B)[RK(C)]
            (7, 0, 1, OpArgMode::OpArgR, OpArgMode::OpArgK, InstMode::iABC),
        );
        m.insert(
            "SETTABUP",
            //	A B C	UpValue[A][RK(B)] := RK(C)
            (8, 0, 0, OpArgMode::OpArgK, OpArgMode::OpArgK, InstMode::iABC),
        );
        m.insert(
            "SETUPVAL",
            //	A B	UpValue[B] := R(A)
            (9, 0, 0, OpArgMode::OpArgU, OpArgMode::OpArgN, InstMode::iABC),
        );
        m.insert(
            "SETTABLE",
            //	A B C	R(A)[RK(B)] := RK(C)
            (10, 0, 0, OpArgMode::OpArgK, OpArgMode::OpArgK, InstMode::iABC),
        );
        m.insert(
            "NEWTABLE",
            //	A B C	R(A) := {} (size = B,C)
            (11, 0, 1, OpArgMode::OpArgU, OpArgMode::OpArgU, InstMode::iABC),
        );
        m.insert(
            "SELF",
            //	A B C	R(A+1) := R(B); R(A) := R(B)[RK(C)]
            (12, 0, 1, OpArgMode::OpArgR, OpArgMode::OpArgK, InstMode::iABC),
        );
        m.insert(
            "ADD",
            //	A B C	R(A) := RK(B) + RK(C)
            (13, 0, 1, OpArgMode::OpArgK, OpArgMode::OpArgK, InstMode::iABC),
        );
        m.insert(
            "SUB",
            //	A B C	R(A) := RK(B) - RK(C)
            (14, 0, 1, OpArgMode::OpArgK, OpArgMode::OpArgK, InstMode::iABC),
        );
        m.insert(
            "MUL",
            //	A B C	R(A) := RK(B) * RK(C)
            (15, 0, 1, OpArgMode::OpArgK, OpArgMode::OpArgK, InstMode::iABC),
        );
        m.insert(
            "DIV",
            //	A B C	R(A) := RK(B) / RK(C)
            (16, 0, 1, OpArgMode::OpArgK, OpArgMode::OpArgK, InstMode::iABC),
        );
        m.insert(
            "MOD",
            //	A B C	R(A) := RK(B) % RK(C)
            (17, 0, 1, OpArgMode::OpArgK, OpArgMode::OpArgK, InstMode::iABC),
        );
        m.insert(
            "POW",
            //	A B C	R(A) := RK(B) ^ RK(C)
            (18, 0, 1, OpArgMode::OpArgK, OpArgMode::OpArgK, InstMode::iABC),
        );
        m.insert(
            "UNM",
            //	A B	R(A) := -R(B)
            (19, 0, 1, OpArgMode::OpArgR, OpArgMode::OpArgN, InstMode::iABC),
        );
        m.insert(
            "NOT",
            //	A B	R(A) := not R(B)
            (20, 0, 1, OpArgMode::OpArgR, OpArgMode::OpArgN, InstMode::iABC),
        );
        m.insert(
            "LEN",
            //	A B	R(A) := length of R(B)
            (21, 0, 1, OpArgMode::OpArgR, OpArgMode::OpArgN, InstMode::iABC),
        );
        m.insert(
            "CONCAT",
            //	A B C	R(A) := R(B).. ... ..R(C)
            (22, 0, 1, OpArgMode::OpArgR, OpArgMode::OpArgR, InstMode::iABC),
        );
        m.insert(
            "JMP",
            //	A sBx	pc+=sBx; if (A) close all upvalues >= R(A - 1)
            (23, 0, 0, OpArgMode::OpArgR, OpArgMode::OpArgN, InstMode::iAsBx),
        );
        m.insert(
            "EQ",
            //	A B C	if ((RK(B) == RK(C)) ~= A) then pc++
            (24, 1, 0, OpArgMode::OpArgK, OpArgMode::OpArgK, InstMode::iABC),
        );
        m.insert(
            "LT",
            //	A B C	if ((RK(B) <  RK(C)) ~= A) then pc++
            (25, 1, 0, OpArgMode::OpArgK, OpArgMode::OpArgK, InstMode::iABC),
        );
        m.insert(
            "LE",
            //	A B C	if ((RK(B) <= RK(C)) ~= A) then pc++
            (26, 1, 0, OpArgMode::OpArgK, OpArgMode::OpArgK, InstMode::iABC),
        );
        m.insert(
            "TEST",
            //	A C	if not (R(A) <=> C) then pc++
            (27, 1, 0, OpArgMode::OpArgN, OpArgMode::OpArgU, InstMode::iABC),
        );
        m.insert(
            "TESTSET",
            //	A B C	if (R(B) <=> C) then R(A) := R(B) else pc++
            (28, 1, 1, OpArgMode::OpArgR, OpArgMode::OpArgU, InstMode::iABC),
        );
        m.insert(
            "CALL",
            //	A B C	R(A), ... ,R(A+C-2) := R(A)(R(A+1), ... ,R(A+B-1))
            (29, 0, 1, OpArgMode::OpArgU, OpArgMode::OpArgU, InstMode::iABC),
        );
        m.insert(
            "TAILCALL",
            //	A B C	return R(A)(R(A+1), ... ,R(A+B-1))
            (30, 0, 1, OpArgMode::OpArgU, OpArgMode::OpArgU, InstMode::iABC),
        );
        m.insert(
            "RETURN",
            //	A B	return R(A), ... ,R(A+B-2)	(see note)
            (31, 0, 0, OpArgMode::OpArgU, OpArgMode::OpArgN, InstMode::iABC),
        );
        m.insert(
            "FORLOOP",
            //	A sBx	R(A)+=R(A+2); #if R(A) <?= R(A+1) then { pc+=sBx; R(A+3)=R(A) }
            (32, 0, 1, OpArgMode::OpArgR, OpArgMode::OpArgN, InstMode::iAsBx),
        );
        m.insert(
            "FORPREP",
            //	A sBx	R(A)-=R(A+2); pc+=sBx
            (33, 0, 1, OpArgMode::OpArgR, OpArgMode::OpArgN, InstMode::iAsBx),
        );
        m.insert(
            "TFORCALL",
            //	A C	R(A+3), ... ,R(A+2+C) := R(A)(R(A+1), R(A+2));
            (34, 0, 0, OpArgMode::OpArgN, OpArgMode::OpArgU, InstMode::iABC),
        );
        m.insert(
            "TFORLOOP",
            //	A sBx	if R(A+1) ~= nil then { R(A)=R(A+1); pc += sBx }
            (35, 0, 1, OpArgMode::OpArgR, OpArgMode::OpArgN, InstMode::iAsBx),
        );
        m.insert(
            "SETLIST",
            //	A B C	R(A)[(C-1)*FPF+i] := R(A+i), 1 <= i <= B
            (36, 0, 0, OpArgMode::OpArgU, OpArgMode::OpArgU, InstMode::iABC),
        );
        m.insert(
            "CLOSURE",
            //	A Bx	R(A) := closure(KPROTO[Bx])
            (37, 0, 1, OpArgMode::OpArgU, OpArgMode::OpArgN, InstMode::iABx),
        );
        m.insert(
            "VARARG",
            //	A B	R(A), R(A+1), ..., R(A+B-2) = vararg
            (38, 0, 1, OpArgMode::OpArgU, OpArgMode::OpArgN, InstMode::iABC),
        );
        m.insert(
            "EXTRAARG",
            //	Ax	extra (larger) argument for previous opcode
            (39, 0, 0, OpArgMode::OpArgU, OpArgMode::OpArgU, InstMode::iAx),
        );
        m
    };
}
