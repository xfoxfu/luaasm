#![allow(dead_code)]

use super::{num_f64, AstCheck, ParseResult};
use crate::writer::{WriteObj, Writer};
use nom::branch::alt;
use nom::bytes::complete::escaped_transform;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::combinator::*;
use nom::sequence::delimited;

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum ConstValue {
    Nil,
    Bool(bool),
    Num(f64),
    Str(String),
}

pub fn const_nil(input: &str) -> ParseResult<ConstValue> {
    value(ConstValue::Nil, tag("nil"))(input)
}
pub fn const_bool(input: &str) -> ParseResult<ConstValue> {
    alt((
        value(ConstValue::Bool(true), tag("true")),
        value(ConstValue::Bool(false), tag("false")),
    ))(input)
}
pub fn const_num(input: &str) -> ParseResult<ConstValue> {
    map(num_f64, ConstValue::Num)(input)
}

pub fn const_string(input: &str) -> ParseResult<ConstValue> {
    map(
        delimited(
            tag("\""),
            escaped_transform(
                is_not("\\\""),
                '\\',
                map(alt((tag("\\"), tag("\""), tag("n"))), |v| match v {
                    "n" => "\n",
                    val => val,
                }),
            ),
            tag("\""),
        ),
        |v| ConstValue::Str(v.to_string()),
    )(input)
}

pub fn const_val(input: &str) -> ParseResult<ConstValue> {
    alt((const_nil, const_bool, const_num, const_string))(input)
}

impl AstCheck for ConstValue {
    fn check(&self) -> Result<(), String> {
        Ok(())
    }
}

impl Into<Vec<u8>> for ConstValue {
    fn into(self) -> Vec<u8> {
        let mut writer = Writer::new();
        match self {
            // [u8 type]
            // type 0: | nil
            ConstValue::Nil => writer.write(0u8),
            ConstValue::Bool(value) => {
                // [u8 type]
                writer.write(1u8);
                // type 1: | bool
                //     [u8 value]
                writer.write(if value { 1u8 } else { 0u8 });
            }
            ConstValue::Num(value) => {
                // [u8 type]
                writer.write(3u8);
                // type 3: | number
                //     [numsize value]
                writer.write(value);
            }
            ConstValue::Str(value) => {
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

#[test]
fn const_val_nil() {
    let (_, res) = const_val("nil").unwrap();
    assert_eq!(res, ConstValue::Nil);
}
#[test]
fn const_val_bool_true() {
    let (_, res) = const_val("true").unwrap();
    assert_eq!(res, ConstValue::Bool(true));
}
#[test]
fn const_val_bool_false() {
    let (_, res) = const_val("false").unwrap();
    assert_eq!(res, ConstValue::Bool(false));
}
#[test]
fn const_val_num_int() {
    let (_, res) = const_val("15\0").unwrap();
    assert_eq!(res, ConstValue::Num(15f64));
}
#[test]
fn const_val_num_float() {
    let (_, res) = const_val("125.7\0").unwrap();
    assert_eq!(res, ConstValue::Num(125.7));
}
#[test]
fn const_val_string() {
    let (_, res) = const_val("\"Hello world!\"\0").unwrap();
    assert_eq!(res, ConstValue::Str("Hello world!".to_string()));
}
#[test]
fn const_val_escape() {
    let (_, res) = const_val("\"Hello \\\"world!\"\0").unwrap();
    assert_eq!(res, ConstValue::Str("Hello \"world!".to_string()));
}
