use super::{Ref, Rule};
use crate::writer::{WriteObj, Writer};
use pest::iterators::Pair;

#[derive(Debug)]
pub struct Upvalue(pub Ref, pub Ref, pub Ref);

impl From<Pair<'_, Rule>> for Upvalue {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let mut pair = pair.into_inner();
        Upvalue(
            Ref::Upvalue(pair.next().unwrap().as_str().split_at(1).1.parse().unwrap()),
            Ref::Stack(pair.next().unwrap().as_str().split_at(1).1.parse().unwrap()),
            Ref::Register(pair.next().unwrap().as_str().split_at(1).1.parse().unwrap()),
        )
    }
}

impl Into<Vec<u8>> for Upvalue {
    fn into(self) -> Vec<u8> {
        let mut writer = Writer::new();
        let Upvalue(_ref, stack, register) = self;
        //         [u8 stack]
        let stack: i32 = stack.into();
        writer.write(stack as u8);
        //         [u8 register]
        let register: i32 = register.into();
        writer.write(register as u8);
        writer.into_inner()
    }
}
