use super::Ref;

#[derive(Debug)]
pub struct Instruction {
    pub op: String,
    pub params: Vec<Ref>,
}
