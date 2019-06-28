#![allow(dead_code)]

use super::{num_u8, ref_register, ref_upvalue, space, Ref};
use crate::writer::{WriteObj, Writer};
use nom::{call, named, tag};

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct UpvalDecl {
    pub id: Ref,
    pub stack_up: u8,
    pub register: Ref,
}

named!(
    pub upval_decl(&str) -> UpvalDecl,
    do_parse!(
        id: ref_upvalue >>
        many0!(space) >>
        tag!("=") >>
        many0!(space) >>
        tag!("L") >>
        stack_up: num_u8 >>
        many0!(space) >>
        register: ref_register >>
        (UpvalDecl { id, stack_up, register })
    )
);

impl Into<Vec<u8>> for UpvalDecl {
    fn into(self) -> Vec<u8> {
        let mut writer = Writer::new();
        let UpvalDecl {
            stack_up, register, ..
        } = self;
        //         [u8 stack]
        writer.write(stack_up);
        //         [u8 register]
        let register: i32 = register.into();
        writer.write(register as u8);
        writer.into_inner()
    }
}

#[test]
fn parse_upval_decl() {
    let (_, res) = upval_decl("U0 = L1 R0\0").unwrap();
    assert_eq!(
        res,
        UpvalDecl {
            id: Ref::Upvalue(0),
            stack_up: 1,
            register: Ref::Register(0),
        }
    );
}
