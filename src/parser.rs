use crate::lua::lua52::LUA_OPCODE;
use crate::lua::{InstMode, OpArgMode};
use crate::writer::{WriteObj, Writer};
use pest::iterators::Pair;

mod constant;
pub use self::constant::Constant;
mod func;
pub use self::func::Func;
mod instruction;
pub use self::instruction::Instruction;
mod luafile;
pub use self::luafile::LuaFile;
mod r#ref;
pub use self::r#ref::Ref;
mod upvalue;
pub use self::upvalue::Upvalue;
mod value;
pub use self::value::Value;

#[derive(Parser)]
#[grammar = "luaasm.pest"] // relative to src
pub struct LuaAsmParser;

pub fn parse_sec_upval(pair: Pair<Rule>) -> Vec<Upvalue> {
    pair.into_inner().map(parse_decl_upval).collect()
}
pub fn parse_decl_upval(pair: Pair<Rule>) -> Upvalue {
    let mut pair = pair.into_inner();
    Upvalue(
        Ref::Upvalue(pair.next().unwrap().as_str().split_at(1).1.parse().unwrap()),
        Ref::Stack(pair.next().unwrap().as_str().split_at(1).1.parse().unwrap()),
        Ref::Register(pair.next().unwrap().as_str().split_at(1).1.parse().unwrap()),
    )
}
pub fn parse_sec_inst(pair: Pair<Rule>) -> Vec<Instruction> {
    pair.into_inner().map(parse_decl_inst).collect()
}
pub fn parse_decl_inst(pair: Pair<Rule>) -> Instruction {
    let mut pair = pair.into_inner();
    let name = pair.next().unwrap().as_str().into();
    Instruction {
        op: name,
        params: pair
            .map(|param| {
                let num: i32 = param
                    .as_str()
                    .parse()
                    .unwrap_or_else(|_| param.as_str().split_at(1).1.parse().unwrap());
                match param.as_rule() {
                    Rule::RefRegister => Ref::Register(num as u32),
                    Rule::RefConst => Ref::Constant(num as u32),
                    Rule::RefUpvalue => Ref::Upvalue(num as u32),
                    Rule::RefImValue => Ref::ImmediateValue(num),
                    _ => unreachable!(),
                }
            })
            .collect(),
    }
}
pub fn parse_sec_func(pair: Pair<Rule>) -> Vec<Func> {
    pair.into_inner().map(parse_fn).collect()
}

impl Into<Vec<u8>> for LuaFile {
    fn into(self) -> Vec<u8> {
        self.0.into()
    }
}
impl Into<Vec<u8>> for Func {
    fn into(self) -> Vec<u8> {
        let mut writer = Writer::new();

        // func:
        //     [int line_start] | debug info
        writer.write(0u32);
        //     [int line_end]   | debug info
        writer.write(0u32);
        //     [u8 nparams]
        writer.write(self.args);
        //     [u8 varargflags]
        writer.write(if self.is_varg { 1u8 } else { 0u8 });
        //     [u8 nregisters]
        writer.write(self.register_count);
        //     [int ninstructions]
        writer.write(self.instructions.len() as u32);
        //     ... instructions:
        //         [instsize instruction]
        for inst in self.instructions {
            let v: u32 = inst.into();
            writer.write(v)
        }
        //     [int nconsts]
        writer.write(self.constants.len() as u32);
        //     ... consts:
        for constant in self.constants {
            let v: Vec<u8> = constant.into();
            writer.write(v)
        }
        //     [int nprimitives]
        writer.write(self.funcs.len() as u32);
        //     ... primitives:
        //         [func primitive]
        for func in self.funcs {
            let v: Vec<u8> = func.into();
            writer.write(v)
        }
        //     [int nupvals]
        writer.write(self.upvalues.len() as u32);
        //     ... upvals:

        //     [string source] | debug info
        //     [int nlines]
        //     ... lines:
        //         [int line]
        //     [int nlocals]
        //     ... locals:
        //         [string name] | debug info
        //         [int startpc]
        //         [int endpc]
        //     [int nupvalnames]
        //     ... upvalnames:
        //         [string name] | debug info
        writer.into_inner()
    }
}
impl Into<u32> for Instruction {
    fn into(self) -> u32 {
        let (opmode_op, _opmode_t, _opmode_a, opmode_b, opmode_c, opmode_inst) =
            LUA_OPCODE.get(self.op.as_str()).expect("invalid op code");
        let mut param_iter = self.params.into_iter();
        let (opa, mut opb, mut opc) = (param_iter.next(), param_iter.next(), param_iter.next());
        // println!("{:?} {:?} {:?}", opa, opb, opc);
        if self.op.as_str() == "TEST" || self.op.as_str() == "TFORCALL" {
            std::mem::swap(&mut opb, &mut opc);
        }

        let offset_b;
        let offset_c;
        if let InstMode::iAsBx = opmode_inst {
            offset_b = 0;
            offset_c = 0
        } else if self.op.as_str() != "LOADK" {
            match opmode_b {
                OpArgMode::OpArgK | OpArgMode::OpArgR => {
                    offset_b = if let Some(Ref::Constant(_)) = opb.as_ref() {
                        0x100
                    } else {
                        0
                    }
                }
                _ => offset_b = 0,
            }
            match opmode_c {
                OpArgMode::OpArgK | OpArgMode::OpArgR => {
                    offset_c = if let Some(Ref::Constant(_)) = opc.as_ref() {
                        0x100
                    } else {
                        0
                    }
                }
                _ => offset_c = 0,
            }
        } else {
            offset_b = 0;
            offset_c = 0
        }

        let (op, a, b, c): (u32, i32, i32, i32) = (
            *opmode_op,
            opa.map(|v| v.into()).unwrap_or_else(|| 0),
            opb.map(|v| {
                let r: i32 = v.into();
                r + offset_b
            })
            .unwrap_or_else(|| 0),
            opc.map(|v| {
                let r: i32 = v.into();
                r + offset_c
            })
            .unwrap_or_else(|| 0),
        );

        let mut val: u32 = op;
        // println!("OP={:>032b}", val);
        match opmode_inst {
            InstMode::iABC => {
                // println!(
                //     "A ={:>032b}\nB ={:>032b}\nC ={:>032b}",
                //     (a as u32) << 6,
                //     (b as u32) << 23,
                //     (c as u32) << 14
                // );
                val |= (a as u32) << 6;
                val |= (c as u32) << 14;
                val |= (b as u32) << 23;
            }
            InstMode::iABx => {
                // println!(
                //     "A ={:>032b}\nB ={:>032b}",
                //     (a as u32) << 6,
                //     (b as u32) << 14
                // );
                val |= (a as u32) << 6;
                val |= (b as u32) << 14;
            }
            InstMode::iAsBx => {
                // println!(
                //     "A ={:>032b}\nB ={:>032b}",
                //     (a as u32) << 6,
                //     (b as u32) << 14
                // );
                val |= (a as u32) << 6;
                val |= ((b - 1 + (1 << 17)) as u32) << 14;
                // val |= (if b >= 0 { 0x01 } else { 0x00 }) << 31;
            }
            InstMode::iAx => {
                // println!("A ={:>032b}", (a as u32) << 6,);
                val |= (a as u32) << 6;
            }
        }
        // println!("{}\n{:>08X}\n{:>032b}\n", self.op, val, val);

        val
    }
}

impl Into<i32> for Ref {
    fn into(self) -> i32 {
        match self {
            Ref::Constant(v) | Ref::Register(v) | Ref::Stack(v) | Ref::Upvalue(v) => v as i32,
            Ref::ImmediateValue(v) => v,
        }
    }
}
impl Into<Vec<u8>> for Constant {
    fn into(self) -> Vec<u8> {
        let Constant(_ref, val) = self;
        val.into()
    }
}
impl Into<Vec<u8>> for Value {
    fn into(self) -> Vec<u8> {
        let mut writer = Writer::new();
        match self {
            // [u8 type]
            // type 0: | nil
            Value::Nil => writer.write(0u8),
            Value::Bool(value) => {
                // [u8 type]
                writer.write(1u8);
                // type 1: | bool
                //     [u8 value]
                writer.write(if value { 1u8 } else { 0u8 });
            }
            Value::Num(value) => {
                // [u8 type]
                writer.write(3u8);
                // type 3: | number
                //     [numsize value]
                writer.write(value);
            }
            Value::Str(value) => {
                // [u8 type]
                writer.write(4u8);
                // type 4: | string
                //     [string value]
                writer.write((value.len() + 1) as u32);
                let chars: Vec<u8> = value.chars().map(|c| c as u8).collect();
                writer.write(chars);
                writer.write(0u8);
            }
        }
        writer.into_inner()
    }
}
impl Into<Vec<u8>> for Upvalue {
    fn into(self) -> Vec<u8> {
        let mut writer = Writer::new();
        let Upvalue(_ref, stack, register) = self;
        //         [u8 stack]
        let stack: i32 = stack.into();
        writer.write(stack as u8);
        //         [u8 register]
        let register: i32 = register.into();
        writer.write(register as u8);
        writer.into_inner()
    }
}
