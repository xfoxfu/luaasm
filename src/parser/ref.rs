#![allow(dead_code)]

use super::{num_i16, num_u8};
use nom::{alt, call, complete, do_parse, error_position, named, tag, types::CompleteStr};

#[derive(Serialize, Debug, PartialEq)]
pub enum Ref {
    Register(u8),
    Const(u8),
    Upvalue(u8),
    Immediate(i16),
}

named!(pub ref_register(CompleteStr) -> Ref,
  do_parse!(
    tag!("R") >>
    id: num_u8 >>
    (Ref::Register(id))
  )
);
named!(pub ref_constant(CompleteStr) -> Ref,
  do_parse!(
    tag!("K") >>
    id: num_u8 >>
    (Ref::Const(id))
  )
);
named!(pub ref_upvalue(CompleteStr) -> Ref,
  do_parse!(
    tag!("U") >>
    id: num_u8 >>
    (Ref::Upvalue(id))
  )
);
named!(pub ref_immediate(CompleteStr) -> Ref,
  map!(num_i16, |v| { Ref::Immediate(v) })
);
named!(pub reference(CompleteStr) -> Ref,
    alt_complete!(ref_register | ref_constant | ref_upvalue | ref_immediate)
);

impl Into<i32> for Ref {
    fn into(self) -> i32 {
        match self {
            Ref::Const(v) | Ref::Register(v) | Ref::Upvalue(v) => i32::from(v),
            Ref::Immediate(v) => i32::from(v),
        }
    }
}

#[test]
fn parse_register() {
    let (_, res) = reference(CompleteStr("R15")).unwrap();
    assert_eq!(res, Ref::Register(15));
}
#[test]
fn parse_const() {
    let (_, res) = reference(CompleteStr("K15")).unwrap();
    assert_eq!(res, Ref::Const(15));
}
#[test]
fn parse_upval() {
    let (_, res) = reference(CompleteStr("U15")).unwrap();
    assert_eq!(res, Ref::Upvalue(15));
}
#[test]
fn parse_immediate_postive() {
    let (_, res) = reference(CompleteStr("15")).unwrap();
    assert_eq!(res, Ref::Immediate(15));
}
#[test]
fn parse_immediate_negative() {
    let (_, res) = reference(CompleteStr("-1")).unwrap();
    assert_eq!(res, Ref::Immediate(-1));
}
#[test]
fn parse_immediate_negative_b() {
    let (_, res) = reference(CompleteStr("15 R0")).unwrap();
    assert_eq!(res, Ref::Immediate(15));
}
