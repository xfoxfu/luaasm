#[derive(Parser)]
#[grammar = "luaasm.pest"] // relative to src
pub struct LuaAsmParser;

mod constant;
pub use self::constant::Constant;
mod func;
pub use self::func::Func;
mod instruction;
pub use self::instruction::Instruction;
mod luafile;
pub use self::luafile::LuaFile;
mod r#ref;
pub use self::r#ref::Ref;
mod upvalue;
pub use self::upvalue::Upvalue;
mod value;
pub use self::value::Value;

