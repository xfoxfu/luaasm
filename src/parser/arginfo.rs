use super::{ref_register, AstCheck, Ref};
use nom::{call, named, tag};

#[derive(Serialize, Debug, PartialEq)]
pub struct ArgInfo {
    pub args: Vec<Ref>,
    pub is_varg: bool,
}

named!(
    pub arg_info(&str) -> ArgInfo,
    map!(ws!(delimited!(
        tag!("("),
        alt_complete!(
            map!(pair!(
                many0!(terminated!(ws!(ref_register), tag!(","))),
                ws!(tag!("__va_args__"))
            ), |(v, _)| (v, true)) |
            map!(ws!(separated_list!(tag!(","), ref_register)), |v| (v, false))
        ),
        tag!(")")
    )), |(args, is_varg)| ArgInfo { args, is_varg })
);

impl AstCheck for ArgInfo {
    fn check(&self) -> Result<(), String> {
        for (i, r) in self.args.iter().enumerate() {
            if let Ref::Register(v) = r {
                if *v != i as u8 {
                    return Err(format!("mismatched register R{} at position {}", v, i));
                }
            } else {
                unreachable!()
            }
        }
        Ok(())
    }
}

#[test]
fn parse_arg_empty() {
    let (_, res) = arg_info("()\0").unwrap();
    assert_eq!(
        res,
        ArgInfo {
            args: vec![],
            is_varg: false
        }
    );
}
#[test]
fn parse_arg_reg() {
    let (_, res) = arg_info("(R0, R1, R2)\0").unwrap();
    assert_eq!(
        res,
        ArgInfo {
            args: vec![Ref::Register(0), Ref::Register(1), Ref::Register(2)],
            is_varg: false
        }
    );
}
#[test]
fn parse_arg_varg() {
    let (_, res) = arg_info("(__va_args__)\0").unwrap();
    assert_eq!(
        res,
        ArgInfo {
            args: vec![],
            is_varg: true
        }
    );
}
#[test]
fn parse_arg_reg_varg() {
    let (_, res) = arg_info("(R0, R1, __va_args__)\0").unwrap();
    assert_eq!(
        res,
        ArgInfo {
            args: vec![Ref::Register(0), Ref::Register(1)],
            is_varg: true
        }
    );
}
