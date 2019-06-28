#![allow(dead_code)]

use super::{reference, Ref};
use crate::lua::{lua52::LUA_OPCODE, InstMode, OpArgMode};
use nom::{alpha, call, named, opt, space};
use serde_derive::Serialize;

#[derive(Serialize, Debug, PartialEq)]
pub struct Instruction {
    pub opcode: String,
    pub args: (Option<Ref>, Option<Ref>, Option<Ref>),
}

named!(pub instruction(&str) -> Instruction,
  do_parse!(
    opcode: map!(alpha, |s| s.to_string()) >>
    many0!(space) >>
    args: tuple!(
        opt!(terminated!(reference, many0!(space))),
        opt!(terminated!(reference, many0!(space))),
        opt!(terminated!(reference, many0!(space)))
    ) >>
    (Instruction { opcode, args })
));

impl Into<u32> for Instruction {
    fn into(self) -> u32 {
        let (opmode_op, _opmode_t, _opmode_a, opmode_b, opmode_c, opmode_inst) = LUA_OPCODE
            .get(self.opcode.as_str())
            .expect("invalid op code");
        let (opa, mut opb, mut opc) = self.args;
        // println!("{:?} {:?} {:?}", opa, opb, opc);
        if self.opcode.as_str() == "TEST" || self.opcode.as_str() == "TFORCALL" {
            std::mem::swap(&mut opb, &mut opc);
        }

        let offset_b;
        let offset_c;
        if let InstMode::iAsBx = opmode_inst {
            offset_b = 0;
            offset_c = 0
        } else if self.opcode.as_str() != "LOADK" {
            match opmode_b {
                OpArgMode::OpArgK | OpArgMode::OpArgR => {
                    offset_b = if let Some(Ref::Const(_)) = opb.as_ref() {
                        0x100
                    } else {
                        0
                    }
                }
                _ => offset_b = 0,
            }
            match opmode_c {
                OpArgMode::OpArgK | OpArgMode::OpArgR => {
                    offset_c = if let Some(Ref::Const(_)) = opc.as_ref() {
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

#[test]
fn parse_instruction() {
    let (_, res) = instruction("GETTABUP R0 U0 K0\0").unwrap();
    assert_eq!(
        res,
        Instruction {
            opcode: "GETTABUP".to_string(),
            args: (
                Some(Ref::Register(0)),
                Some(Ref::Upvalue(0)),
                Some(Ref::Const(0))
            )
        }
    );
}
#[test]
fn parse_instruction_2() {
    let (_, res) = instruction("GETTABUP R0 U0 -15\0").unwrap();
    assert_eq!(
        res,
        Instruction {
            opcode: "GETTABUP".to_string(),
            args: (
                Some(Ref::Register(0)),
                Some(Ref::Upvalue(0)),
                Some(Ref::Immediate(-15))
            )
        }
    );
}
#[test]
fn parse_instruction_3() {
    let (_, res) = instruction("GETTABUP R0 U0\0").unwrap();
    assert_eq!(
        res,
        Instruction {
            opcode: "GETTABUP".to_string(),
            args: (Some(Ref::Register(0)), Some(Ref::Upvalue(0)), None)
        }
    );
}
