#[macro_use]
mod whitespace;
use self::whitespace::{comment, space, space_or_comment};

mod r#ref;
pub use self::r#ref::Ref;
#[allow(unused_imports)]
use self::r#ref::{ref_constant, ref_immediate, ref_register, ref_upvalue, reference};

mod inst;
use self::inst::instruction;
pub use self::inst::Instruction;

mod constdecl;
use self::constdecl::const_decl;
pub use self::constdecl::ConstDecl;

mod constval;
pub use self::constval::ConstValue;
#[allow(unused_imports)]
use self::constval::{const_bool, const_nil, const_num, const_string, const_val};

mod upvaldecl;
#[allow(unused_imports)]
use self::upvaldecl::upval_decl;
pub use self::upvaldecl::UpvalDecl;

mod func;
#[allow(unused_imports)]
use self::func::func_decl;
pub use self::func::Func;

mod file;
pub use self::file::{parse_file, File};

use nom::{digit, recognize_float};
named!(integer_str(&str) -> &str, recognize!(
    pair!(
        opt!(alt!(tag!("+") | tag!("-"))),
        digit
)));
named!(num_u8(&str)-> u8, flat_map!(recognize_float, parse_to!(u8)));
named!(num_u32(&str)-> u32, flat_map!(recognize_float, parse_to!(u32)));
named!(num_i16(&str)-> i16, flat_map!(recognize_float, parse_to!(i16)));
named!(num_i32(&str)-> i32, flat_map!(recognize_float, parse_to!(i32)));
named!(num_f64(&str)-> f64, flat_map!(recognize_float, parse_to!(f64)));

#[test]
fn parse_i32() {
    let (_, res) = num_i32("15;").unwrap();
    assert_eq!(res, 15);
}
#[test]
fn parse_i32_negative() {
    let (_, res) = num_i32("-15;").unwrap();
    assert_eq!(res, -15);
}
