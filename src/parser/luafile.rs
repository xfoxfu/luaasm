use super::{Func, Rule};
use pest::iterators::Pair;

#[derive(Debug)]
pub struct LuaFile(pub Func);

impl From<Pair<'_, Rule>> for LuaFile {
    fn from(pest: Pair<'_, Rule>) -> LuaFile {
        LuaFile(pest.into_inner().next().unwrap().into())
    }
}

impl Into<Vec<u8>> for LuaFile {
    fn into(self) -> Vec<u8> {
        self.0.into()
    }
}
