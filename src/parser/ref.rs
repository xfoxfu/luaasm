#[derive(Debug)]
pub enum Ref {
    Register(u32),
    Constant(u32),
    Upvalue(u32),
    ImmediateValue(i32),
    Stack(u32),
}