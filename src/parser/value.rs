use crate::writer::{WriteObj, Writer};

#[derive(Debug)]
pub enum Value {
    Nil,
    Bool(bool),
    Num(f64),
    Str(String),
}

impl Into<Vec<u8>> for Value {
    fn into(self) -> Vec<u8> {
        let mut writer = Writer::new();
        match self {
            // [u8 type]
            // type 0: | nil
            Value::Nil => writer.write(0u8),
            Value::Bool(value) => {
                // [u8 type]
                writer.write(1u8);
                // type 1: | bool
                //     [u8 value]
                writer.write(if value { 1u8 } else { 0u8 });
            }
            Value::Num(value) => {
                // [u8 type]
                writer.write(3u8);
                // type 3: | number
                //     [numsize value]
                writer.write(value);
            }
            Value::Str(value) => {
                // [u8 type]
                writer.write(4u8);
                // type 4: | string
                //     [string value]
                writer.write((value.len() + 1) as u32);
                let chars: Vec<u8> = value.chars().map(|c| c as u8).collect();
                writer.write(chars);
                writer.write(0u8);
            }
        }
        writer.into_inner()
    }
}
