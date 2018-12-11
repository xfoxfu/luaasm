#[derive(Debug)]
pub enum Ref {
    Register(u32),
    Constant(u32),
    Upvalue(u32),
    ImmediateValue(i32),
    Stack(u32),
}

impl Into<i32> for Ref {
    fn into(self) -> i32 {
        match self {
            Ref::Constant(v) | Ref::Register(v) | Ref::Stack(v) | Ref::Upvalue(v) => v as i32,
            Ref::ImmediateValue(v) => v,
        }
    }
}
