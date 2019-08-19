#![allow(dead_code)]

use super::ParseResult;
use super::{num_i16, num_u8};
use nom::branch::alt;
use nom::bytes::complete::*;
use nom::combinator::*;
use serde_derive::Serialize;

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum Ref {
    Register(u8),
    Const(u8),
    Upvalue(u8),
    Immediate(i16),
}

pub fn ref_register(input: &str) -> ParseResult<Ref> {
    let (input, _) = tag("R")(input)?;
    let (input, id) = num_u8(input)?;
    Ok((input, Ref::Register(id)))
}
pub fn ref_constant(input: &str) -> ParseResult<Ref> {
    let (input, _) = tag("K")(input)?;
    let (input, id) = num_u8(input)?;
    Ok((input, Ref::Const(id)))
}
pub fn ref_upvalue(input: &str) -> ParseResult<Ref> {
    let (input, _) = tag("U")(input)?;
    let (input, id) = num_u8(input)?;
    Ok((input, Ref::Upvalue(id)))
}
pub fn ref_immediate(input: &str) -> ParseResult<Ref> {
    map(num_i16, |v| Ref::Immediate(v))(input)
}
pub fn reference(input: &str) -> ParseResult<Ref> {
    alt((ref_register, ref_constant, ref_upvalue, ref_immediate))(input)
}

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
    let (_, res) = reference("R15;").unwrap();
    assert_eq!(res, Ref::Register(15));
}
#[test]
fn parse_const() {
    let (_, res) = reference("K15;").unwrap();
    assert_eq!(res, Ref::Const(15));
}
#[test]
fn parse_upval() {
    let (_, res) = reference("U15;").unwrap();
    assert_eq!(res, Ref::Upvalue(15));
}
#[test]
fn parse_immediate_postive() {
    let (_, res) = reference("15;").unwrap();
    assert_eq!(res, Ref::Immediate(15));
}
#[test]
fn parse_immediate_negative() {
    let (_, res) = reference("-1;").unwrap();
    assert_eq!(res, Ref::Immediate(-1));
}
#[test]
fn parse_immediate_negative_b() {
    let (_, res) = reference("15 R0").unwrap();
    assert_eq!(res, Ref::Immediate(15));
}
