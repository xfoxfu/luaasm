#[derive(Debug)]
pub enum Value {
    Nil,
    Bool(bool),
    Num(f64),
    Str(String),
}