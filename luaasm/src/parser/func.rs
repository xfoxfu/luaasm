#![allow(dead_code)]

use super::ParseResult;
use super::{
    arg_info, const_decl, instruction, space, space_or_comment, upval_decl, ArgInfo, AstCheck,
    ConstDecl, Instruction, UpvalDecl,
};
use crate::writer::{WriteObj, Writer};
use nom::bytes::complete::*;
use nom::multi::many0;
use nom::sequence::*;

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct Func {
    pub arg_info: ArgInfo,
    pub register_count: u8,
    pub constants: Vec<ConstDecl>,
    pub upvalues: Vec<UpvalDecl>,
    pub instructions: Vec<Instruction>,
    pub funcs: Vec<Func>,
}

fn count_registers(insts: &[Instruction]) -> u8 {
    use super::Ref;

    let mut count = 0u8;
    for inst in insts {
        let mut handle = |arg: &Option<Ref>| {
            if let Some(Ref::Register(id)) = arg {
                if *id as u8 > count {
                    count = *id as u8
                }
            }
        };
        let (arg_1, arg_2, arg_3) = &inst.args;
        handle(arg_1);
        handle(arg_2);
        handle(arg_3);
    }
    count + 1
}

pub fn func_decl(input: &str) -> ParseResult<Func> {
    let (input, _) = many0(space_or_comment)(input)?;
    let (input, (arg_info, instructions, constants, upvalues, funcs, _)) = tuple((
        delimited(
            terminated(tag(".fn"), many0(space)),
            arg_info,
            many0(space_or_comment),
        ),
        preceded(
            terminated(tag(".instruction"), many0(space_or_comment)),
            many0(terminated(instruction, many0(space_or_comment))),
        ),
        preceded(
            terminated(tag(".const"), many0(space_or_comment)),
            many0(terminated(const_decl, many0(space_or_comment))),
        ),
        preceded(
            terminated(tag(".upvalue"), many0(space_or_comment)),
            many0(terminated(upval_decl, many0(space_or_comment))),
        ),
        many0(terminated(func_decl, many0(space_or_comment))),
        tag(".endfn"),
    ))(input)?;
    Ok((
        input,
        Func {
            arg_info,
            register_count: count_registers(&instructions),
            constants,
            upvalues,
            instructions,
            funcs,
        },
    ))
}

impl AstCheck for Func {
    fn check(&self) -> Result<(), String> {
        self.arg_info.check() // TODO: check instructions
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
        writer.write(self.arg_info.args.len() as u8);
        //     [u8 varargflags]
        writer.write(if self.arg_info.is_varg { 1u8 } else { 0u8 });
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
        for upval in self.upvalues {
            let v: Vec<u8> = upval.into();
            writer.write(v);
        }

        //     [string source] | debug info
        writer.write(0u32);
        //     [int nlines]
        writer.write(0u32);
        //     ... lines:
        //         [int line]
        //     [int nlocals]
        writer.write(0u32);
        //     ... locals:
        //         [string name] | debug info
        //         [int startpc]
        //         [int endpc]
        //     [int nupvalnames]
        writer.write(0u32);
        //     ... upvalnames:
        //         [string name] | debug info
        writer.into_inner()
    }
}
