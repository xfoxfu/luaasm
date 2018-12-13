use super::ref_register;
use nom::{call, named, tag};

#[derive(Serialize, Debug, PartialEq)]
pub struct ArgInfo {
    pub args: u8,
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
    )), |(v_reg, is_varg)| ArgInfo { args: v_reg.len() as u8, is_varg })
);

#[test]
fn parse_arg_empty() {
    let (_, res) = arg_info("()\0").unwrap();
    assert_eq!(
        res,
        ArgInfo {
            args: 0,
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
            args: 3,
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
            args: 0,
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
            args: 2,
            is_varg: true
        }
    );
}
