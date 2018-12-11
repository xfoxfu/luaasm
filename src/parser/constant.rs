use super::{Ref, Rule, Value};
use pest::iterators::Pair;

#[derive(Debug)]
pub struct Constant(pub Ref, pub Value);

impl From<Pair<'_, Rule>> for Constant {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let mut pair = pair.into_inner();
        let refstr = pair.next().unwrap().as_str();
        let refnum = refstr.split_at(1).1.parse().unwrap();
        let value = match pair.next().unwrap().as_str() {
            "nil" => Value::Nil,
            "bool" => match pair.next().unwrap().as_str() {
                "True" => Value::Bool(true),
                "False" => Value::Bool(false),
                _ => unreachable!(),
            },
            "string" => Value::Str(pair.next().unwrap().as_str().into()),
            "number" => Value::Num(pair.next().unwrap().as_str().parse().unwrap()),
            _ => unreachable!(),
        };
        Constant(Ref::Constant(refnum), value)
    }
}

impl Into<Vec<u8>> for Constant {
    fn into(self) -> Vec<u8> {
        let Constant(_ref, val) = self;
        val.into()
    }
}
