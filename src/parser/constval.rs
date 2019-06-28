#![allow(dead_code)]

use super::{num_f64, AstCheck};
use crate::writer::{WriteObj, Writer};
use nom::{
    alt, call, delimited, error_node_position, error_position, escaped_transform, is_not, map,
    named, tag, tuple_parser, types::CompleteStr, value,
};

#[derive(Serialize, Debug, PartialEq)]
pub enum ConstValue {
    Nil,
    Bool(bool),
    Num(f64),
    Str(String),
}

named!(
    pub const_nil(CompleteStr) -> ConstValue,
    value!(ConstValue::Nil, tag!("nil"))
);
named!(
    pub const_bool(CompleteStr) -> ConstValue,
    alt!(
        value!(ConstValue::Bool(true), tag!("true")) | 
        value!(ConstValue::Bool(false), tag!("false"))
));
named!(
    pub const_num(CompleteStr) -> ConstValue,
    map!(num_f64, ConstValue::Num)
);
named!(
    pub const_string(CompleteStr) -> ConstValue,
    map!(delimited!(
        tag!("\""),
        escaped_transform!(is_not!("\\\""), '\\', alt!(
            tag!("\\") => { |_| "\\" } |
            tag!("\"") => { |_| "\"" } |
            tag!("n")  => { |_| "\n" }
        )),
        tag!("\"")
    ), |v| ConstValue::Str(v.to_string()))
);

named!(
    pub const_val(CompleteStr) -> ConstValue,
    alt!(const_nil | const_bool | const_num | const_string)
);

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
    let (_, res) = const_val(CompleteStr("nil")).unwrap();
    assert_eq!(res, ConstValue::Nil);
}
#[test]
fn const_val_bool_true() {
    let (_, res) = const_val(CompleteStr("true")).unwrap();
    assert_eq!(res, ConstValue::Bool(true));
}
#[test]
fn const_val_bool_false() {
    let (_, res) = const_val(CompleteStr("false")).unwrap();
    assert_eq!(res, ConstValue::Bool(false));
}
#[test]
fn const_val_num_int() {
    let (_, res) = const_val(CompleteStr("15\0")).unwrap();
    assert_eq!(res, ConstValue::Num(15f64));
}
#[test]
fn const_val_num_float() {
    let (_, res) = const_val(CompleteStr("125.7\0")).unwrap();
    assert_eq!(res, ConstValue::Num(125.7));
}
#[test]
fn const_val_string() {
    let (_, res) = const_val(CompleteStr("\"Hello world!\"\0")).unwrap();
    assert_eq!(res, ConstValue::Str("Hello world!".to_string()));
}
#[test]
fn const_val_escape() {
    let (_, res) = const_val(CompleteStr("\"Hello \\\"world!\"\0")).unwrap();
    assert_eq!(res, ConstValue::Str("Hello \"world!".to_string()));
}
