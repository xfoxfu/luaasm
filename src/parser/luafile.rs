use super::{Func, Rule};
use pest::iterators::Pair;

#[derive(Debug)]
pub struct LuaFile(pub Func);

impl From<Pair<'_, Rule>> for LuaFile {
    fn from(pest: Pair<'_, Rule>) -> LuaFile {
        LuaFile(pest.into_inner().next().unwrap().into())
    }
}
