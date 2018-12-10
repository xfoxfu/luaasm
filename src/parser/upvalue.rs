use super::Ref;

#[derive(Debug)]
pub struct Upvalue(pub Ref, pub Ref, pub Ref);
