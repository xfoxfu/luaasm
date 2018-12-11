use super::{Ref, Rule};
use crate::lua::{lua52::LUA_OPCODE, InstMode, OpArgMode};
use pest::iterators::Pair;

#[derive(Debug)]
pub struct Instruction {
    pub op: String,
    pub params: Vec<Ref>,
}

impl From<Pair<'_, Rule>> for Instruction {
    fn from(pair: Pair<'_, Rule>) -> Self {
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
