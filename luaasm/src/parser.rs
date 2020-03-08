mod number;
#[allow(unused_imports)]
use self::number::{num_f64, num_i16, num_u8};

#[macro_use]
mod whitespace;
#[allow(unused_imports)]
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

mod arginfo;
#[allow(unused_imports)]
use self::arginfo::arg_info;
pub use self::arginfo::ArgInfo;

mod func;
#[allow(unused_imports)]
use self::func::func_decl;
pub use self::func::Func;

mod file;
pub use self::file::{parse_file, File};

pub trait AstCheck {
    fn check(&self) -> Result<(), String>;
}

pub type ParseResult<'a, T> = nom::IResult<&'a str, T, nom::error::VerboseError<&'a str>>;
