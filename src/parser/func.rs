use super::{Constant, Instruction, Ref, Rule, Upvalue};
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
        let upvalues = parse_sec_upval(pair.next().unwrap());
        let instructions = parse_sec_inst(pair.next().unwrap());
        let funcs = parse_sec_func(pair.next().unwrap());
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
