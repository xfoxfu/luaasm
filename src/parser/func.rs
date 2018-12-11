use super::{Constant, Instruction, Ref, Rule, Upvalue};
use crate::writer::{WriteObj, Writer};
use pest::iterators::Pair;

#[derive(Debug)]
pub struct Func {
    pub args: u8,
    pub is_varg: bool,
    pub register_count: u8,
    pub constants: Vec<Constant>,
    pub upvalues: Vec<Upvalue>,
    pub instructions: Vec<Instruction>,
    pub funcs: Vec<Func>,
}

impl From<Pair<'_, Rule>> for Func {
    fn from(pair: Pair<'_, Rule>) -> Func {
        let mut pair = pair.into_inner();
        let args = pair.next().unwrap().as_str().parse().unwrap();
        let is_varg = match pair.next().unwrap().as_str() {
            "true" => true,
            "false" => false,
            s => panic!("invalid varg={}", s),
        };
        let constants = pair
            .next()
            .unwrap()
            .into_inner()
            .map(|c| c.into())
            .collect();
        let upvalues = pair
            .next()
            .unwrap()
            .into_inner()
            .map(|c| c.into())
            .collect();
        let instructions: Vec<Instruction> = pair
            .next()
            .unwrap()
            .into_inner()
            .map(|c| c.into())
            .collect();
        let funcs = pair
            .next()
            .unwrap()
            .into_inner()
            .map(|c| c.into())
            .collect();
        Func {
            args,
            is_varg,
            register_count: count_registers(&instructions),
            constants,
            upvalues,
            instructions,
            funcs,
        }
    }
}
fn count_registers(insts: &[Instruction]) -> u8 {
    let mut count = 0u8;
    for inst in insts {
        for p in &inst.params {
            if let Ref::Register(v) = p {
                if *v as u8 > count {
                    count = *v as u8;
                }
            }
        }
    }
    count + 1
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
