#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
extern crate nom;
#[macro_use]
extern crate serde_derive;
use clap::App;
mod app {
    pub mod asm {
        use crate::parser::AstCheck;
        use crate::writer::{WriteObj, Writer};
        use clap::{App, Arg, ArgMatches, SubCommand};
        use nom::error::convert_error;
        use nom::Err;
        use std::fs::File;
        use std::io::Read;
        pub fn get_subcommand() -> App<'static, 'static> {
            SubCommand::with_name("asm")
                .about("assemble lua bytecode")
                .arg(Arg::with_name("input").required(true))
                .arg(
                    Arg::with_name("output")
                        .required(true)
                        .default_value("luac.out"),
                )
                .arg(
                    Arg::with_name("lua")
                        .short("l")
                        .long("lua")
                        .takes_value(true)
                        .default_value("5.2"),
                )
                .arg(
                    Arg::with_name("endian")
                        .short("e")
                        .long("endian")
                        .takes_value(true)
                        .default_value("little"),
                )
        }
        pub fn run(args: &ArgMatches) {
            let mut file = args
                .value_of("input")
                .and_then(|path| File::open(path).ok())
                .expect("cannot open file");
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("cannot read file content");
            match crate::parser::parse_file(&contents) {
                Err(Err::Error(e)) | Err(Err::Failure(e)) => {
                    {
                        ::std::io::_print(::core::fmt::Arguments::new_v1(
                            &["An error occurred when parsing:\n", "\n"],
                            &match (&convert_error(contents.as_str(), e),) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ));
                    };
                    return;
                }
                Ok((_, file)) => {
                    file.check().unwrap();
                    let mut writer = Writer::new();
                    let content: Vec<u8> = file.into();
                    writer.write(content);
                    let mut file = args
                        .value_of("output")
                        .and_then(|path| File::create(path).ok())
                        .expect("cannot open write file");
                    writer
                        .write_to_file(&mut file)
                        .expect("cannot write output");
                }
                _ => ::std::rt::begin_panic("internal error: entered unreachable code"),
            }
        }
    }
}
mod lua {
    #![allow(dead_code)]
    #[macro_use]
    mod helper {}
    pub mod lua52 {
        #![allow(unused_assignments)]
        use super::{InstMode, OpArgMode, Target};
        use luaasm_derive::{impl_datatype, impl_opcode};
        pub struct Lua52;
        impl Target for Lua52 {
            fn datatype(datatype: &str) -> usize {
                match datatype {
                    "LUA_TNONE" => -1,
                    "LUA_TNIL" => 0,
                    "LUA_TBOOLEAN" => 1,
                    "LUA_TLIGHTUSERDATA" => 2,
                    "LUA_TNUMBER" => 3,
                    "LUA_TSTRING" => 4,
                    "LUA_TTABLE" => 5,
                    "LUA_TFUNCTION" => 6,
                    "LUA_TUSERDATA" => 7,
                    "LUA_TTHREAD" => 8,
                    "LUA_NUMTYPES" => 9,
                    _ => ::std::rt::begin_panic("invalid opcode"),
                }
            }
            fn opcode(opcode: &str) -> (u32, u8, u8, OpArgMode, OpArgMode, InstMode) {
                match opcode {
                    "_MOVE" => (
                        0u32,
                        0,
                        1,
                        OpArgMode::OpArgR,
                        OpArgMode::OpArgN,
                        InstMode::iABC,
                    ),
                    "_LOADK" => (
                        1u32,
                        0,
                        1,
                        OpArgMode::OpArgK,
                        OpArgMode::OpArgN,
                        InstMode::iABx,
                    ),
                    "_LOADKX" => (
                        2u32,
                        0,
                        1,
                        OpArgMode::OpArgN,
                        OpArgMode::OpArgN,
                        InstMode::iABx,
                    ),
                    "_LOADBOOL" => (
                        3u32,
                        0,
                        1,
                        OpArgMode::OpArgU,
                        OpArgMode::OpArgU,
                        InstMode::iABC,
                    ),
                    "_LOADNIL" => (
                        4u32,
                        0,
                        1,
                        OpArgMode::OpArgU,
                        OpArgMode::OpArgN,
                        InstMode::iABC,
                    ),
                    "_GETUPVAL" => (
                        5u32,
                        0,
                        1,
                        OpArgMode::OpArgU,
                        OpArgMode::OpArgN,
                        InstMode::iABC,
                    ),
                    "_GETTABUP" => (
                        6u32,
                        0,
                        1,
                        OpArgMode::OpArgU,
                        OpArgMode::OpArgK,
                        InstMode::iABC,
                    ),
                    "_GETTABLE" => (
                        7u32,
                        0,
                        1,
                        OpArgMode::OpArgR,
                        OpArgMode::OpArgK,
                        InstMode::iABC,
                    ),
                    "_SETTABUP" => (
                        8u32,
                        0,
                        0,
                        OpArgMode::OpArgK,
                        OpArgMode::OpArgK,
                        InstMode::iABC,
                    ),
                    "_SETUPVAL" => (
                        9u32,
                        0,
                        0,
                        OpArgMode::OpArgU,
                        OpArgMode::OpArgN,
                        InstMode::iABC,
                    ),
                    "_SETTABLE" => (
                        10u32,
                        0,
                        0,
                        OpArgMode::OpArgK,
                        OpArgMode::OpArgK,
                        InstMode::iABC,
                    ),
                    "_NEWTABLE" => (
                        11u32,
                        0,
                        1,
                        OpArgMode::OpArgU,
                        OpArgMode::OpArgU,
                        InstMode::iABC,
                    ),
                    "_SELF" => (
                        12u32,
                        0,
                        1,
                        OpArgMode::OpArgR,
                        OpArgMode::OpArgK,
                        InstMode::iABC,
                    ),
                    "_ADD" => (
                        13u32,
                        0,
                        1,
                        OpArgMode::OpArgK,
                        OpArgMode::OpArgK,
                        InstMode::iABC,
                    ),
                    "_SUB" => (
                        14u32,
                        0,
                        1,
                        OpArgMode::OpArgK,
                        OpArgMode::OpArgK,
                        InstMode::iABC,
                    ),
                    "_MUL" => (
                        15u32,
                        0,
                        1,
                        OpArgMode::OpArgK,
                        OpArgMode::OpArgK,
                        InstMode::iABC,
                    ),
                    "_DIV" => (
                        16u32,
                        0,
                        1,
                        OpArgMode::OpArgK,
                        OpArgMode::OpArgK,
                        InstMode::iABC,
                    ),
                    "_MOD" => (
                        17u32,
                        0,
                        1,
                        OpArgMode::OpArgK,
                        OpArgMode::OpArgK,
                        InstMode::iABC,
                    ),
                    "_POW" => (
                        18u32,
                        0,
                        1,
                        OpArgMode::OpArgK,
                        OpArgMode::OpArgK,
                        InstMode::iABC,
                    ),
                    "_UNM" => (
                        19u32,
                        0,
                        1,
                        OpArgMode::OpArgR,
                        OpArgMode::OpArgN,
                        InstMode::iABC,
                    ),
                    "_NOT" => (
                        20u32,
                        0,
                        1,
                        OpArgMode::OpArgR,
                        OpArgMode::OpArgN,
                        InstMode::iABC,
                    ),
                    "_LEN" => (
                        21u32,
                        0,
                        1,
                        OpArgMode::OpArgR,
                        OpArgMode::OpArgN,
                        InstMode::iABC,
                    ),
                    "_CONCAT" => (
                        22u32,
                        0,
                        1,
                        OpArgMode::OpArgR,
                        OpArgMode::OpArgR,
                        InstMode::iABC,
                    ),
                    "_JMP" => (
                        23u32,
                        0,
                        0,
                        OpArgMode::OpArgR,
                        OpArgMode::OpArgN,
                        InstMode::iAsBx,
                    ),
                    "_EQ" => (
                        24u32,
                        1,
                        0,
                        OpArgMode::OpArgK,
                        OpArgMode::OpArgK,
                        InstMode::iABC,
                    ),
                    "_LT" => (
                        25u32,
                        1,
                        0,
                        OpArgMode::OpArgK,
                        OpArgMode::OpArgK,
                        InstMode::iABC,
                    ),
                    "_LE" => (
                        26u32,
                        1,
                        0,
                        OpArgMode::OpArgK,
                        OpArgMode::OpArgK,
                        InstMode::iABC,
                    ),
                    "_TEST" => (
                        27u32,
                        1,
                        0,
                        OpArgMode::OpArgN,
                        OpArgMode::OpArgU,
                        InstMode::iABC,
                    ),
                    "_TESTSET" => (
                        28u32,
                        1,
                        1,
                        OpArgMode::OpArgR,
                        OpArgMode::OpArgU,
                        InstMode::iABC,
                    ),
                    "_CALL" => (
                        29u32,
                        0,
                        1,
                        OpArgMode::OpArgU,
                        OpArgMode::OpArgU,
                        InstMode::iABC,
                    ),
                    "_TAILCALL" => (
                        30u32,
                        0,
                        1,
                        OpArgMode::OpArgU,
                        OpArgMode::OpArgU,
                        InstMode::iABC,
                    ),
                    "_RETURN" => (
                        31u32,
                        0,
                        0,
                        OpArgMode::OpArgU,
                        OpArgMode::OpArgN,
                        InstMode::iABC,
                    ),
                    "_FORLOOP" => (
                        32u32,
                        0,
                        1,
                        OpArgMode::OpArgR,
                        OpArgMode::OpArgN,
                        InstMode::iAsBx,
                    ),
                    "_FORPREP" => (
                        33u32,
                        0,
                        1,
                        OpArgMode::OpArgR,
                        OpArgMode::OpArgN,
                        InstMode::iAsBx,
                    ),
                    "_TFORCALL" => (
                        34u32,
                        0,
                        0,
                        OpArgMode::OpArgN,
                        OpArgMode::OpArgU,
                        InstMode::iABC,
                    ),
                    "_TFORLOOP" => (
                        35u32,
                        0,
                        1,
                        OpArgMode::OpArgR,
                        OpArgMode::OpArgN,
                        InstMode::iAsBx,
                    ),
                    "_SETLIST" => (
                        36u32,
                        0,
                        0,
                        OpArgMode::OpArgU,
                        OpArgMode::OpArgU,
                        InstMode::iABC,
                    ),
                    "_CLOSURE" => (
                        37u32,
                        0,
                        1,
                        OpArgMode::OpArgU,
                        OpArgMode::OpArgN,
                        InstMode::iABx,
                    ),
                    "_VARARG" => (
                        38u32,
                        0,
                        1,
                        OpArgMode::OpArgU,
                        OpArgMode::OpArgN,
                        InstMode::iABC,
                    ),
                    "_EXTRAARG" => (
                        39u32,
                        0,
                        0,
                        OpArgMode::OpArgU,
                        OpArgMode::OpArgU,
                        InstMode::iAx,
                    ),
                    _ => ::std::rt::begin_panic("invalid opcode"),
                }
            }
        }
    }
    pub use lua52::Lua52;
    pub enum OpArgMode {
        OpArgN,
        OpArgU,
        OpArgR,
        OpArgK,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for OpArgMode {
        #[inline]
        fn clone(&self) -> OpArgMode {
            match (&*self,) {
                (&OpArgMode::OpArgN,) => OpArgMode::OpArgN,
                (&OpArgMode::OpArgU,) => OpArgMode::OpArgU,
                (&OpArgMode::OpArgR,) => OpArgMode::OpArgR,
                (&OpArgMode::OpArgK,) => OpArgMode::OpArgK,
            }
        }
    }
    pub enum InstMode {
        #[allow(non_camel_case_types)]
        iABC,
        #[allow(non_camel_case_types)]
        iABx,
        #[allow(non_camel_case_types)]
        iAsBx,
        #[allow(non_camel_case_types)]
        iAx,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for InstMode {
        #[inline]
        fn clone(&self) -> InstMode {
            match (&*self,) {
                (&InstMode::iABC,) => InstMode::iABC,
                (&InstMode::iABx,) => InstMode::iABx,
                (&InstMode::iAsBx,) => InstMode::iAsBx,
                (&InstMode::iAx,) => InstMode::iAx,
            }
        }
    }
    pub enum LuaVersion {
        Lua52,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for LuaVersion {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&LuaVersion::Lua52,) => {
                    let mut debug_trait_builder = f.debug_tuple("Lua52");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    pub enum Endian {
        BigEndian,
        LittleEndian,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Endian {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Endian::BigEndian,) => {
                    let mut debug_trait_builder = f.debug_tuple("BigEndian");
                    debug_trait_builder.finish()
                }
                (&Endian::LittleEndian,) => {
                    let mut debug_trait_builder = f.debug_tuple("LittleEndian");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    pub trait Target {
        fn datatype(datatype: &str) -> usize;
        fn opcode(opcode: &str) -> (u32, u8, u8, OpArgMode, OpArgMode, InstMode);
    }
}
mod parser {
    mod number {
        use super::ParseResult;
        use nom::combinator::*;
        use nom::number::complete::recognize_float;
        pub fn num_u8(input: &str) -> ParseResult<u8> {
            map_res(recognize_float, |s: &str| s.parse::<u8>())(input)
        }
        pub fn num_i16(input: &str) -> ParseResult<i16> {
            map_res(recognize_float, |s: &str| s.parse::<i16>())(input)
        }
        pub fn num_f64(input: &str) -> ParseResult<f64> {
            map_res(recognize_float, |s: &str| s.parse::<f64>())(input)
        }
    }
    #[allow(unused_imports)]
    use self::number::{num_f64, num_i16, num_u8};
    #[macro_use]
    mod whitespace {
        use super::ParseResult;
        use nom::branch::*;
        use nom::bytes::complete::*;
        use nom::character::complete::*;
        use nom::character::complete::{line_ending, not_line_ending};
        use nom::sequence::*;
        pub fn space(input: &str) -> ParseResult<()> {
            let (input, _) = one_of(" \t\n\r")(input)?;
            Ok((input, ()))
        }
        pub fn comment(input: &str) -> ParseResult<()> {
            let (input, _) = delimited(tag(";"), not_line_ending, line_ending)(input)?;
            Ok((input, ()))
        }
        pub fn space_or_comment(input: &str) -> ParseResult<()> {
            let (input, _) = alt((space, comment))(input)?;
            Ok((input, ()))
        }
    }
    #[allow(unused_imports)]
    use self::whitespace::{comment, space, space_or_comment};
    mod r#ref {
        #![allow(dead_code)]
        use super::ParseResult;
        use super::{num_i16, num_u8};
        use nom::branch::alt;
        use nom::bytes::complete::*;
        use nom::combinator::*;
        use serde_derive::Serialize;
        pub enum Ref {
            Register(u8),
            Const(u8),
            Upvalue(u8),
            Immediate(i16),
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_SERIALIZE_FOR_Ref: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Ref {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::export::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        Ref::Register(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Ref",
                                0u32,
                                "Register",
                                __field0,
                            )
                        }
                        Ref::Const(ref __field0) => _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "Ref",
                            1u32,
                            "Const",
                            __field0,
                        ),
                        Ref::Upvalue(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Ref",
                                2u32,
                                "Upvalue",
                                __field0,
                            )
                        }
                        Ref::Immediate(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Ref",
                                3u32,
                                "Immediate",
                                __field0,
                            )
                        }
                    }
                }
            }
        };
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for Ref {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match (&*self,) {
                    (&Ref::Register(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Register");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&Ref::Const(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Const");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&Ref::Upvalue(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Upvalue");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&Ref::Immediate(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Immediate");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for Ref {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for Ref {
            #[inline]
            fn eq(&self, other: &Ref) -> bool {
                {
                    let __self_vi =
                        unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
                    let __arg_1_vi =
                        unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            (&Ref::Register(ref __self_0), &Ref::Register(ref __arg_1_0)) => {
                                (*__self_0) == (*__arg_1_0)
                            }
                            (&Ref::Const(ref __self_0), &Ref::Const(ref __arg_1_0)) => {
                                (*__self_0) == (*__arg_1_0)
                            }
                            (&Ref::Upvalue(ref __self_0), &Ref::Upvalue(ref __arg_1_0)) => {
                                (*__self_0) == (*__arg_1_0)
                            }
                            (&Ref::Immediate(ref __self_0), &Ref::Immediate(ref __arg_1_0)) => {
                                (*__self_0) == (*__arg_1_0)
                            }
                            _ => unsafe { ::core::intrinsics::unreachable() },
                        }
                    } else {
                        false
                    }
                }
            }
            #[inline]
            fn ne(&self, other: &Ref) -> bool {
                {
                    let __self_vi =
                        unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
                    let __arg_1_vi =
                        unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            (&Ref::Register(ref __self_0), &Ref::Register(ref __arg_1_0)) => {
                                (*__self_0) != (*__arg_1_0)
                            }
                            (&Ref::Const(ref __self_0), &Ref::Const(ref __arg_1_0)) => {
                                (*__self_0) != (*__arg_1_0)
                            }
                            (&Ref::Upvalue(ref __self_0), &Ref::Upvalue(ref __arg_1_0)) => {
                                (*__self_0) != (*__arg_1_0)
                            }
                            (&Ref::Immediate(ref __self_0), &Ref::Immediate(ref __arg_1_0)) => {
                                (*__self_0) != (*__arg_1_0)
                            }
                            _ => unsafe { ::core::intrinsics::unreachable() },
                        }
                    } else {
                        true
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for Ref {
            #[inline]
            fn clone(&self) -> Ref {
                match (&*self,) {
                    (&Ref::Register(ref __self_0),) => {
                        Ref::Register(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&Ref::Const(ref __self_0),) => {
                        Ref::Const(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&Ref::Upvalue(ref __self_0),) => {
                        Ref::Upvalue(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&Ref::Immediate(ref __self_0),) => {
                        Ref::Immediate(::core::clone::Clone::clone(&(*__self_0)))
                    }
                }
            }
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
    }
    pub use self::r#ref::Ref;
    #[allow(unused_imports)]
    use self::r#ref::{ref_constant, ref_immediate, ref_register, ref_upvalue, reference};
    mod inst {
        #![allow(dead_code)]
        use super::ParseResult;
        use super::{reference, Ref};
        use crate::lua::{InstMode, Lua52, OpArgMode, Target};
        use nom::character::complete::*;
        use nom::combinator::*;
        use nom::sequence::*;
        use serde_derive::Serialize;
        pub struct Instruction {
            pub opcode: String,
            pub args: (Option<Ref>, Option<Ref>, Option<Ref>),
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_SERIALIZE_FOR_Instruction: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Instruction {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::export::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "Instruction",
                        false as usize + 1 + 1,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "opcode",
                        &self.opcode,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "args",
                        &self.args,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for Instruction {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Instruction {
                        opcode: ref __self_0_0,
                        args: ref __self_0_1,
                    } => {
                        let mut debug_trait_builder = f.debug_struct("Instruction");
                        let _ = debug_trait_builder.field("opcode", &&(*__self_0_0));
                        let _ = debug_trait_builder.field("args", &&(*__self_0_1));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for Instruction {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for Instruction {
            #[inline]
            fn eq(&self, other: &Instruction) -> bool {
                match *other {
                    Instruction {
                        opcode: ref __self_1_0,
                        args: ref __self_1_1,
                    } => match *self {
                        Instruction {
                            opcode: ref __self_0_0,
                            args: ref __self_0_1,
                        } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &Instruction) -> bool {
                match *other {
                    Instruction {
                        opcode: ref __self_1_0,
                        args: ref __self_1_1,
                    } => match *self {
                        Instruction {
                            opcode: ref __self_0_0,
                            args: ref __self_0_1,
                        } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for Instruction {
            #[inline]
            fn clone(&self) -> Instruction {
                match *self {
                    Instruction {
                        opcode: ref __self_0_0,
                        args: ref __self_0_1,
                    } => Instruction {
                        opcode: ::core::clone::Clone::clone(&(*__self_0_0)),
                        args: ::core::clone::Clone::clone(&(*__self_0_1)),
                    },
                }
            }
        }
        pub fn instruction(input: &str) -> ParseResult<Instruction> {
            let (input, opcode) = map(alpha1, |s: &str| s.to_string())(input)?;
            let (input, _) = space0(input)?;
            let (input, args) = tuple((
                opt(terminated(reference, space0)),
                opt(terminated(reference, space0)),
                opt(terminated(reference, space0)),
            ))(input)?;
            Ok((input, Instruction { opcode, args }))
        }
        impl Into<u32> for Instruction {
            fn into(self) -> u32 {
                let (opmode_op, _opmode_t, _opmode_a, opmode_b, opmode_c, opmode_inst) =
                    Lua52::opcode(self.opcode.as_str());
                let (opa, mut opb, mut opc) = self.args;
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
                    opmode_op,
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
                match opmode_inst {
                    InstMode::iABC => {
                        val |= (a as u32) << 6;
                        val |= (c as u32) << 14;
                        val |= (b as u32) << 23;
                    }
                    InstMode::iABx => {
                        val |= (a as u32) << 6;
                        val |= (b as u32) << 14;
                    }
                    InstMode::iAsBx => {
                        val |= (a as u32) << 6;
                        val |= ((b - 1 + (1 << 17)) as u32) << 14;
                    }
                    InstMode::iAx => {
                        val |= (a as u32) << 6;
                    }
                }
                val
            }
        }
    }
    use self::inst::instruction;
    pub use self::inst::Instruction;
    mod constdecl {
        #![allow(dead_code)]
        use super::ParseResult;
        use super::{const_val, ref_constant, AstCheck, ConstValue, Ref};
        use nom::bytes::complete::*;
        use nom::character::complete::*;
        use nom::sequence::*;
        use serde_derive::Serialize;
        pub struct ConstDecl {
            pub id: Ref,
            pub value: ConstValue,
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_SERIALIZE_FOR_ConstDecl: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ConstDecl {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::export::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "ConstDecl",
                        false as usize + 1 + 1,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "value",
                        &self.value,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for ConstDecl {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    ConstDecl {
                        id: ref __self_0_0,
                        value: ref __self_0_1,
                    } => {
                        let mut debug_trait_builder = f.debug_struct("ConstDecl");
                        let _ = debug_trait_builder.field("id", &&(*__self_0_0));
                        let _ = debug_trait_builder.field("value", &&(*__self_0_1));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for ConstDecl {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for ConstDecl {
            #[inline]
            fn eq(&self, other: &ConstDecl) -> bool {
                match *other {
                    ConstDecl {
                        id: ref __self_1_0,
                        value: ref __self_1_1,
                    } => match *self {
                        ConstDecl {
                            id: ref __self_0_0,
                            value: ref __self_0_1,
                        } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &ConstDecl) -> bool {
                match *other {
                    ConstDecl {
                        id: ref __self_1_0,
                        value: ref __self_1_1,
                    } => match *self {
                        ConstDecl {
                            id: ref __self_0_0,
                            value: ref __self_0_1,
                        } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for ConstDecl {
            #[inline]
            fn clone(&self) -> ConstDecl {
                match *self {
                    ConstDecl {
                        id: ref __self_0_0,
                        value: ref __self_0_1,
                    } => ConstDecl {
                        id: ::core::clone::Clone::clone(&(*__self_0_0)),
                        value: ::core::clone::Clone::clone(&(*__self_0_1)),
                    },
                }
            }
        }
        pub fn const_decl(input: &str) -> ParseResult<ConstDecl> {
            let (input, _) = space0(input)?;
            let (input, id) = ref_constant(input)?;
            let (input, _) = space0(input)?;
            let (input, _) = tag("=")(input)?;
            let (input, value) = delimited(space0, const_val, space0)(input)?;
            Ok((input, ConstDecl { id, value }))
        }
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
    }
    use self::constdecl::const_decl;
    pub use self::constdecl::ConstDecl;
    mod constval {
        #![allow(dead_code)]
        use super::{num_f64, AstCheck, ParseResult};
        use crate::writer::{WriteObj, Writer};
        use nom::branch::alt;
        use nom::bytes::complete::escaped_transform;
        use nom::bytes::complete::is_not;
        use nom::bytes::complete::tag;
        use nom::combinator::*;
        use nom::sequence::delimited;
        pub enum ConstValue {
            Nil,
            Bool(bool),
            Num(f64),
            Str(String),
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_SERIALIZE_FOR_ConstValue: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ConstValue {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::export::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        ConstValue::Nil => _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ConstValue",
                            0u32,
                            "Nil",
                        ),
                        ConstValue::Bool(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "ConstValue",
                                1u32,
                                "Bool",
                                __field0,
                            )
                        }
                        ConstValue::Num(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "ConstValue",
                                2u32,
                                "Num",
                                __field0,
                            )
                        }
                        ConstValue::Str(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "ConstValue",
                                3u32,
                                "Str",
                                __field0,
                            )
                        }
                    }
                }
            }
        };
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for ConstValue {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match (&*self,) {
                    (&ConstValue::Nil,) => {
                        let mut debug_trait_builder = f.debug_tuple("Nil");
                        debug_trait_builder.finish()
                    }
                    (&ConstValue::Bool(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Bool");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&ConstValue::Num(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Num");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&ConstValue::Str(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Str");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for ConstValue {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for ConstValue {
            #[inline]
            fn eq(&self, other: &ConstValue) -> bool {
                {
                    let __self_vi =
                        unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
                    let __arg_1_vi =
                        unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            (&ConstValue::Bool(ref __self_0), &ConstValue::Bool(ref __arg_1_0)) => {
                                (*__self_0) == (*__arg_1_0)
                            }
                            (&ConstValue::Num(ref __self_0), &ConstValue::Num(ref __arg_1_0)) => {
                                (*__self_0) == (*__arg_1_0)
                            }
                            (&ConstValue::Str(ref __self_0), &ConstValue::Str(ref __arg_1_0)) => {
                                (*__self_0) == (*__arg_1_0)
                            }
                            _ => true,
                        }
                    } else {
                        false
                    }
                }
            }
            #[inline]
            fn ne(&self, other: &ConstValue) -> bool {
                {
                    let __self_vi =
                        unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
                    let __arg_1_vi =
                        unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            (&ConstValue::Bool(ref __self_0), &ConstValue::Bool(ref __arg_1_0)) => {
                                (*__self_0) != (*__arg_1_0)
                            }
                            (&ConstValue::Num(ref __self_0), &ConstValue::Num(ref __arg_1_0)) => {
                                (*__self_0) != (*__arg_1_0)
                            }
                            (&ConstValue::Str(ref __self_0), &ConstValue::Str(ref __arg_1_0)) => {
                                (*__self_0) != (*__arg_1_0)
                            }
                            _ => false,
                        }
                    } else {
                        true
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for ConstValue {
            #[inline]
            fn clone(&self) -> ConstValue {
                match (&*self,) {
                    (&ConstValue::Nil,) => ConstValue::Nil,
                    (&ConstValue::Bool(ref __self_0),) => {
                        ConstValue::Bool(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&ConstValue::Num(ref __self_0),) => {
                        ConstValue::Num(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&ConstValue::Str(ref __self_0),) => {
                        ConstValue::Str(::core::clone::Clone::clone(&(*__self_0)))
                    }
                }
            }
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
                    ConstValue::Nil => writer.write(0u8),
                    ConstValue::Bool(value) => {
                        writer.write(1u8);
                        writer.write(if value { 1u8 } else { 0u8 });
                    }
                    ConstValue::Num(value) => {
                        writer.write(3u8);
                        writer.write(value);
                    }
                    ConstValue::Str(value) => {
                        writer.write(4u8);
                        writer.write((value.len() + 1) as u32);
                        let chars: Vec<u8> = value.chars().map(|c| c as u8).collect();
                        writer.write(chars);
                        writer.write(0u8);
                    }
                }
                writer.into_inner()
            }
        }
    }
    pub use self::constval::ConstValue;
    #[allow(unused_imports)]
    use self::constval::{const_bool, const_nil, const_num, const_string, const_val};
    mod upvaldecl {
        #![allow(dead_code)]
        use super::ParseResult;
        use super::{num_u8, ref_register, ref_upvalue, Ref};
        use crate::writer::{WriteObj, Writer};
        use nom::bytes::complete::tag;
        use nom::character::complete::*;
        pub struct UpvalDecl {
            pub id: Ref,
            pub stack_up: u8,
            pub register: Ref,
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_SERIALIZE_FOR_UpvalDecl: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for UpvalDecl {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::export::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "UpvalDecl",
                        false as usize + 1 + 1 + 1,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "stack_up",
                        &self.stack_up,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "register",
                        &self.register,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for UpvalDecl {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    UpvalDecl {
                        id: ref __self_0_0,
                        stack_up: ref __self_0_1,
                        register: ref __self_0_2,
                    } => {
                        let mut debug_trait_builder = f.debug_struct("UpvalDecl");
                        let _ = debug_trait_builder.field("id", &&(*__self_0_0));
                        let _ = debug_trait_builder.field("stack_up", &&(*__self_0_1));
                        let _ = debug_trait_builder.field("register", &&(*__self_0_2));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for UpvalDecl {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for UpvalDecl {
            #[inline]
            fn eq(&self, other: &UpvalDecl) -> bool {
                match *other {
                    UpvalDecl {
                        id: ref __self_1_0,
                        stack_up: ref __self_1_1,
                        register: ref __self_1_2,
                    } => match *self {
                        UpvalDecl {
                            id: ref __self_0_0,
                            stack_up: ref __self_0_1,
                            register: ref __self_0_2,
                        } => {
                            (*__self_0_0) == (*__self_1_0)
                                && (*__self_0_1) == (*__self_1_1)
                                && (*__self_0_2) == (*__self_1_2)
                        }
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &UpvalDecl) -> bool {
                match *other {
                    UpvalDecl {
                        id: ref __self_1_0,
                        stack_up: ref __self_1_1,
                        register: ref __self_1_2,
                    } => match *self {
                        UpvalDecl {
                            id: ref __self_0_0,
                            stack_up: ref __self_0_1,
                            register: ref __self_0_2,
                        } => {
                            (*__self_0_0) != (*__self_1_0)
                                || (*__self_0_1) != (*__self_1_1)
                                || (*__self_0_2) != (*__self_1_2)
                        }
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for UpvalDecl {
            #[inline]
            fn clone(&self) -> UpvalDecl {
                match *self {
                    UpvalDecl {
                        id: ref __self_0_0,
                        stack_up: ref __self_0_1,
                        register: ref __self_0_2,
                    } => UpvalDecl {
                        id: ::core::clone::Clone::clone(&(*__self_0_0)),
                        stack_up: ::core::clone::Clone::clone(&(*__self_0_1)),
                        register: ::core::clone::Clone::clone(&(*__self_0_2)),
                    },
                }
            }
        }
        pub fn upval_decl(input: &str) -> ParseResult<UpvalDecl> {
            let (input, id) = ref_upvalue(input)?;
            let (input, _) = space0(input)?;
            let (input, _) = tag("=")(input)?;
            let (input, _) = space0(input)?;
            let (input, _) = tag("L")(input)?;
            let (input, stack_up) = num_u8(input)?;
            let (input, _) = space0(input)?;
            let (input, register) = ref_register(input)?;
            Ok((
                input,
                UpvalDecl {
                    id,
                    stack_up,
                    register,
                },
            ))
        }
        impl Into<Vec<u8>> for UpvalDecl {
            fn into(self) -> Vec<u8> {
                let mut writer = Writer::new();
                let UpvalDecl {
                    stack_up, register, ..
                } = self;
                writer.write(stack_up);
                let register: i32 = register.into();
                writer.write(register as u8);
                writer.into_inner()
            }
        }
    }
    #[allow(unused_imports)]
    use self::upvaldecl::upval_decl;
    pub use self::upvaldecl::UpvalDecl;
    mod arginfo {
        use super::ParseResult;
        use super::{ref_register, AstCheck, Ref};
        use nom::branch::alt;
        use nom::bytes::complete::*;
        use nom::character::complete::*;
        use nom::combinator::*;
        use nom::multi::{many0, separated_list};
        use nom::sequence::*;
        use serde_derive::Serialize;
        pub struct ArgInfo {
            pub args: Vec<Ref>,
            pub is_varg: bool,
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_SERIALIZE_FOR_ArgInfo: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ArgInfo {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::export::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "ArgInfo",
                        false as usize + 1 + 1,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "args",
                        &self.args,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "is_varg",
                        &self.is_varg,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for ArgInfo {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    ArgInfo {
                        args: ref __self_0_0,
                        is_varg: ref __self_0_1,
                    } => {
                        let mut debug_trait_builder = f.debug_struct("ArgInfo");
                        let _ = debug_trait_builder.field("args", &&(*__self_0_0));
                        let _ = debug_trait_builder.field("is_varg", &&(*__self_0_1));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for ArgInfo {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for ArgInfo {
            #[inline]
            fn eq(&self, other: &ArgInfo) -> bool {
                match *other {
                    ArgInfo {
                        args: ref __self_1_0,
                        is_varg: ref __self_1_1,
                    } => match *self {
                        ArgInfo {
                            args: ref __self_0_0,
                            is_varg: ref __self_0_1,
                        } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &ArgInfo) -> bool {
                match *other {
                    ArgInfo {
                        args: ref __self_1_0,
                        is_varg: ref __self_1_1,
                    } => match *self {
                        ArgInfo {
                            args: ref __self_0_0,
                            is_varg: ref __self_0_1,
                        } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for ArgInfo {
            #[inline]
            fn clone(&self) -> ArgInfo {
                match *self {
                    ArgInfo {
                        args: ref __self_0_0,
                        is_varg: ref __self_0_1,
                    } => ArgInfo {
                        args: ::core::clone::Clone::clone(&(*__self_0_0)),
                        is_varg: ::core::clone::Clone::clone(&(*__self_0_1)),
                    },
                }
            }
        }
        pub fn arg_info(input: &str) -> ParseResult<ArgInfo> {
            map(
                delimited(
                    tag("("),
                    alt((
                        map(
                            pair(
                                many0(delimited(space0, ref_register, tag(","))),
                                preceded(space0, tag("__va_args__")),
                            ),
                            |(v, _)| (v, true),
                        ),
                        map(
                            separated_list(tag(","), delimited(space0, ref_register, space0)),
                            |v| (v, false),
                        ),
                    )),
                    tag(")"),
                ),
                |(args, is_varg)| ArgInfo { args, is_varg },
            )(input)
        }
        impl AstCheck for ArgInfo {
            fn check(&self) -> Result<(), String> {
                for (i, r) in self.args.iter().enumerate() {
                    if let Ref::Register(v) = r {
                        if *v != i as u8 {
                            return Err({
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &["mismatched register R", " at position "],
                                    &match (&v, &i) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ));
                                res
                            });
                        }
                    } else {
                        {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code")
                            }
                        }
                    }
                }
                Ok(())
            }
        }
    }
    #[allow(unused_imports)]
    use self::arginfo::arg_info;
    pub use self::arginfo::ArgInfo;
    mod func {
        #![allow(dead_code)]
        use super::ParseResult;
        use super::{
            arg_info, const_decl, instruction, space, space_or_comment, upval_decl, ArgInfo,
            AstCheck, ConstDecl, Instruction, UpvalDecl,
        };
        use crate::writer::{WriteObj, Writer};
        use nom::bytes::complete::*;
        use nom::multi::many0;
        use nom::sequence::*;
        pub struct Func {
            pub arg_info: ArgInfo,
            pub register_count: u8,
            pub constants: Vec<ConstDecl>,
            pub upvalues: Vec<UpvalDecl>,
            pub instructions: Vec<Instruction>,
            pub funcs: Vec<Func>,
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_SERIALIZE_FOR_Func: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Func {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::export::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "Func",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "arg_info",
                        &self.arg_info,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "register_count",
                        &self.register_count,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "constants",
                        &self.constants,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "upvalues",
                        &self.upvalues,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "instructions",
                        &self.instructions,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "funcs",
                        &self.funcs,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for Func {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Func {
                        arg_info: ref __self_0_0,
                        register_count: ref __self_0_1,
                        constants: ref __self_0_2,
                        upvalues: ref __self_0_3,
                        instructions: ref __self_0_4,
                        funcs: ref __self_0_5,
                    } => {
                        let mut debug_trait_builder = f.debug_struct("Func");
                        let _ = debug_trait_builder.field("arg_info", &&(*__self_0_0));
                        let _ = debug_trait_builder.field("register_count", &&(*__self_0_1));
                        let _ = debug_trait_builder.field("constants", &&(*__self_0_2));
                        let _ = debug_trait_builder.field("upvalues", &&(*__self_0_3));
                        let _ = debug_trait_builder.field("instructions", &&(*__self_0_4));
                        let _ = debug_trait_builder.field("funcs", &&(*__self_0_5));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for Func {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for Func {
            #[inline]
            fn eq(&self, other: &Func) -> bool {
                match *other {
                    Func {
                        arg_info: ref __self_1_0,
                        register_count: ref __self_1_1,
                        constants: ref __self_1_2,
                        upvalues: ref __self_1_3,
                        instructions: ref __self_1_4,
                        funcs: ref __self_1_5,
                    } => match *self {
                        Func {
                            arg_info: ref __self_0_0,
                            register_count: ref __self_0_1,
                            constants: ref __self_0_2,
                            upvalues: ref __self_0_3,
                            instructions: ref __self_0_4,
                            funcs: ref __self_0_5,
                        } => {
                            (*__self_0_0) == (*__self_1_0)
                                && (*__self_0_1) == (*__self_1_1)
                                && (*__self_0_2) == (*__self_1_2)
                                && (*__self_0_3) == (*__self_1_3)
                                && (*__self_0_4) == (*__self_1_4)
                                && (*__self_0_5) == (*__self_1_5)
                        }
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &Func) -> bool {
                match *other {
                    Func {
                        arg_info: ref __self_1_0,
                        register_count: ref __self_1_1,
                        constants: ref __self_1_2,
                        upvalues: ref __self_1_3,
                        instructions: ref __self_1_4,
                        funcs: ref __self_1_5,
                    } => match *self {
                        Func {
                            arg_info: ref __self_0_0,
                            register_count: ref __self_0_1,
                            constants: ref __self_0_2,
                            upvalues: ref __self_0_3,
                            instructions: ref __self_0_4,
                            funcs: ref __self_0_5,
                        } => {
                            (*__self_0_0) != (*__self_1_0)
                                || (*__self_0_1) != (*__self_1_1)
                                || (*__self_0_2) != (*__self_1_2)
                                || (*__self_0_3) != (*__self_1_3)
                                || (*__self_0_4) != (*__self_1_4)
                                || (*__self_0_5) != (*__self_1_5)
                        }
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for Func {
            #[inline]
            fn clone(&self) -> Func {
                match *self {
                    Func {
                        arg_info: ref __self_0_0,
                        register_count: ref __self_0_1,
                        constants: ref __self_0_2,
                        upvalues: ref __self_0_3,
                        instructions: ref __self_0_4,
                        funcs: ref __self_0_5,
                    } => Func {
                        arg_info: ::core::clone::Clone::clone(&(*__self_0_0)),
                        register_count: ::core::clone::Clone::clone(&(*__self_0_1)),
                        constants: ::core::clone::Clone::clone(&(*__self_0_2)),
                        upvalues: ::core::clone::Clone::clone(&(*__self_0_3)),
                        instructions: ::core::clone::Clone::clone(&(*__self_0_4)),
                        funcs: ::core::clone::Clone::clone(&(*__self_0_5)),
                    },
                }
            }
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
            let (input, (arg_info, instructions, constants, upvalues, funcs, _)) =
                tuple((
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
                self.arg_info.check()
            }
        }
        impl Into<Vec<u8>> for Func {
            fn into(self) -> Vec<u8> {
                let mut writer = Writer::new();
                writer.write(0u32);
                writer.write(0u32);
                writer.write(self.arg_info.args.len() as u8);
                writer.write(if self.arg_info.is_varg { 1u8 } else { 0u8 });
                writer.write(self.register_count);
                writer.write(self.instructions.len() as u32);
                for inst in self.instructions {
                    let v: u32 = inst.into();
                    writer.write(v)
                }
                writer.write(self.constants.len() as u32);
                for constant in self.constants {
                    let v: Vec<u8> = constant.into();
                    writer.write(v)
                }
                writer.write(self.funcs.len() as u32);
                for func in self.funcs {
                    let v: Vec<u8> = func.into();
                    writer.write(v)
                }
                writer.write(self.upvalues.len() as u32);
                for upval in self.upvalues {
                    let v: Vec<u8> = upval.into();
                    writer.write(v);
                }
                writer.write(0u32);
                writer.write(0u32);
                writer.write(0u32);
                writer.write(0u32);
                writer.into_inner()
            }
        }
    }
    #[allow(unused_imports)]
    use self::func::func_decl;
    pub use self::func::Func;
    mod file {
        #![allow(dead_code)]
        use super::ParseResult;
        use super::{func_decl, AstCheck, Func};
        use crate::writer::{WriteObj, Writer};
        use nom::combinator::*;
        pub struct File {
            pub main: Func,
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_SERIALIZE_FOR_File: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for File {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::export::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "File",
                        false as usize + 1,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "main",
                        &self.main,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for File {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    File {
                        main: ref __self_0_0,
                    } => {
                        let mut debug_trait_builder = f.debug_struct("File");
                        let _ = debug_trait_builder.field("main", &&(*__self_0_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for File {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for File {
            #[inline]
            fn eq(&self, other: &File) -> bool {
                match *other {
                    File {
                        main: ref __self_1_0,
                    } => match *self {
                        File {
                            main: ref __self_0_0,
                        } => (*__self_0_0) == (*__self_1_0),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &File) -> bool {
                match *other {
                    File {
                        main: ref __self_1_0,
                    } => match *self {
                        File {
                            main: ref __self_0_0,
                        } => (*__self_0_0) != (*__self_1_0),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for File {
            #[inline]
            fn clone(&self) -> File {
                match *self {
                    File {
                        main: ref __self_0_0,
                    } => File {
                        main: ::core::clone::Clone::clone(&(*__self_0_0)),
                    },
                }
            }
        }
        pub fn parse_file(input: &str) -> ParseResult<File> {
            map(func_decl, |f| File { main: f })(input)
        }
        impl AstCheck for File {
            fn check(&self) -> Result<(), String> {
                self.main.check()
            }
        }
        impl Into<Vec<u8>> for File {
            fn into(self) -> Vec<u8> {
                let mut writer = Writer::new();
                writer.write(0x1Bu8);
                writer.write(0x4Cu8);
                writer.write(0x75u8);
                writer.write(0x61u8);
                writer.write(0x52u8);
                writer.write(0x00u8);
                writer.write(0x01u8);
                writer.write(0x04u8);
                writer.write(0x04u8);
                writer.write(0x04u8);
                writer.write(0x08u8);
                writer.write(0x00u8);
                writer.write(0x19u8);
                writer.write(0x93u8);
                writer.write(0x0Du8);
                writer.write(0x0Au8);
                writer.write(0x1Au8);
                writer.write(0x0Au8);
                let main: Vec<u8> = self.main.into();
                writer.write(main);
                writer.into_inner()
            }
        }
    }
    pub use self::file::{parse_file, File};
    pub trait AstCheck {
        fn check(&self) -> Result<(), String>;
    }
    pub type ParseResult<'a, T> = nom::IResult<&'a str, T, nom::error::VerboseError<&'a str>>;
}
mod writer {
    #![allow(dead_code)]
    use std::io::Write;
    pub struct Writer {
        vec: Vec<u8>,
        big_endian: bool,
    }
    impl Writer {
        pub fn new() -> Self {
            Writer {
                vec: Vec::new(),
                big_endian: false,
            }
        }
        pub fn big_endian(self, big_endian: bool) -> Self {
            Writer {
                vec: self.vec,
                big_endian,
            }
        }
        pub fn into_inner(self) -> Vec<u8> {
            self.vec
        }
        pub fn vec_mut(&mut self) -> &mut Vec<u8> {
            &mut self.vec
        }
        pub fn write_to_file(&self, file: &mut std::fs::File) -> Result<(), std::io::Error> {
            file.write_all(&self.vec)
        }
        pub fn write_u8(&mut self, num: u8) {
            self.vec_mut().push(num)
        }
    }
    pub trait WriteObj<T> {
        fn write(&mut self, obj: T);
    }
    impl WriteObj<u8> for Writer {
        fn write(&mut self, num: u8) {
            self.write_u8(num)
        }
    }
    impl WriteObj<u32> for Writer {
        fn write(&mut self, num: u32) {
            if self.big_endian {
                self.vec_mut().push(((num & (0xFF << 24)) >> 24) as u8);
                self.vec_mut().push(((num & (0xFF << 16)) >> 16) as u8);
                self.vec_mut().push(((num & (0xFF << 8)) >> 8) as u8);
                self.vec_mut().push((num & 0xFF) as u8);
            } else {
                self.vec_mut().push((num & 0xFF) as u8);
                self.vec_mut().push(((num & (0xFF << 8)) >> 8) as u8);
                self.vec_mut().push(((num & (0xFF << 16)) >> 16) as u8);
                self.vec_mut().push(((num & (0xFF << 24)) >> 24) as u8);
            }
        }
    }
    impl WriteObj<u64> for Writer {
        fn write(&mut self, num: u64) {
            if self.big_endian {
                self.vec_mut().push(((num & (0xFF << 56)) >> 56) as u8);
                self.vec_mut().push(((num & (0xFF << 48)) >> 48) as u8);
                self.vec_mut().push(((num & (0xFF << 40)) >> 40) as u8);
                self.vec_mut().push(((num & (0xFF << 32)) >> 32) as u8);
                self.vec_mut().push(((num & (0xFF << 24)) >> 24) as u8);
                self.vec_mut().push(((num & (0xFF << 16)) >> 16) as u8);
                self.vec_mut().push(((num & (0xFF << 8)) >> 8) as u8);
                self.vec_mut().push((num & 0xFF) as u8);
            } else {
                self.vec_mut().push((num & 0xFF) as u8);
                self.vec_mut().push(((num & (0xFF << 8)) >> 8) as u8);
                self.vec_mut().push(((num & (0xFF << 16)) >> 16) as u8);
                self.vec_mut().push(((num & (0xFF << 24)) >> 24) as u8);
                self.vec_mut().push(((num & (0xFF << 32)) >> 32) as u8);
                self.vec_mut().push(((num & (0xFF << 40)) >> 40) as u8);
                self.vec_mut().push(((num & (0xFF << 48)) >> 48) as u8);
                self.vec_mut().push(((num & (0xFF << 56)) >> 56) as u8);
            }
        }
    }
    impl WriteObj<f64> for Writer {
        fn write(&mut self, num: f64) {
            self.write(num.to_bits())
        }
    }
    impl WriteObj<Vec<u8>> for Writer {
        fn write(&mut self, mut num: Vec<u8>) {
            self.vec_mut().append(&mut num)
        }
    }
    pub trait WriteTo {
        fn write_to(self, target: &mut Writer);
    }
    impl<T> WriteTo for T
    where
        Writer: WriteObj<T>,
    {
        fn write_to(self, target: &mut Writer) {
            target.write(self)
        }
    }
}
fn main() {
    let app: clap::App = App::new("luaasm")
        .version("1.0")
        .author("coderfox <i@xfox.me>")
        .about("Lua bytecode assembler")
        .subcommand(app::asm::get_subcommand());
    let matches = app.get_matches();
    #[allow(clippy::single_match)]
    match matches.subcommand() {
        ("asm", Some(asm)) => app::asm::run(asm),
        _ => (),
    }
}
