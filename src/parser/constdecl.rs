#![allow(dead_code)]

use super::{const_val, ref_constant, space, AstCheck, ConstValue, Ref};
use nom::{call, named, tag};
use serde_derive::Serialize;

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct ConstDecl {
    pub id: Ref,
    pub value: ConstValue,
}

named!(
    pub const_decl(&str) -> ConstDecl,
    do_parse!(
        id: delimited!(many0!(space), ref_constant, many0!(space)) >>
        tag!("=") >>
        value: delimited!(many0!(space), const_val, many0!(space)) >>
        (ConstDecl { id, value })
    )
);

impl AstCheck for ConstDecl {
    fn check(&self) -> Result<(), String> {
        Ok(())
    }
}

impl Into<Vec<u8>> for ConstDecl {
    fn into(self) -> Vec<u8> {
        let ConstDecl { value, .. } = self;
        value.into()
    }
}

#[test]
fn const_decl_nil() {
    let (_, res) = const_decl("K1 = nil\0").unwrap();
    assert_eq!(
        res,
        ConstDecl {
            id: Ref::Const(1),
            value: ConstValue::Nil
        }
    );
}
#[test]
fn const_decl_bool_true() {
    let (_, res) = const_decl("K1 = true\0").unwrap();
    assert_eq!(
        res,
        ConstDecl {
            id: Ref::Const(1),
            value: ConstValue::Bool(true)
        }
    );
}
#[test]
fn const_decl_bool_false() {
    let (_, res) = const_decl("K1 = false\0").unwrap();
    assert_eq!(
        res,
        ConstDecl {
            id: Ref::Const(1),
            value: ConstValue::Bool(false)
        }
    );
}
#[test]
fn const_decl_num_int() {
    let (_, res) = const_decl("K1 = 15\0").unwrap();
    assert_eq!(
        res,
        ConstDecl {
            id: Ref::Const(1),
            value: ConstValue::Num(15f64)
        }
    );
}
#[test]
fn const_decl_num_float() {
    let (_, res) = const_decl("K1 = 125.7\0").unwrap();
    assert_eq!(
        res,
        ConstDecl {
            id: Ref::Const(1),
            value: ConstValue::Num(125.7)
        }
    );
}
#[test]
fn const_decl_string() {
    let (_, res) = const_decl("K1 = \"Hello world!\"\0").unwrap();
    assert_eq!(
        res,
        ConstDecl {
            id: Ref::Const(1),
            value: ConstValue::Str("Hello world!".to_string())
        }
    );
}
#[test]
fn const_decl_escape() {
    let (_, res) = const_decl("K1 = \"Hello \\\"world!\"\0").unwrap();
    assert_eq!(
        res,
        ConstDecl {
            id: Ref::Const(1),
            value: ConstValue::Str("Hello \"world!".to_string())
        }
    );
}
