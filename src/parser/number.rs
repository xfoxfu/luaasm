use nom::{digit, recognize_float, types::CompleteStr};
named!(integer_str(&str) -> &str, recognize!(
    pair!(
        opt!(alt!(tag!("+") | tag!("-"))),
        digit
)));
named!(pub num_u8(CompleteStr)-> u8, flat_map!(recognize_float, parse_to!(u8)));
named!(pub num_u32(CompleteStr)-> u32, flat_map!(recognize_float, parse_to!(u32)));
named!(pub num_i16(CompleteStr)-> i16, flat_map!(recognize_float, parse_to!(i16)));
named!(pub num_i32(CompleteStr)-> i32, flat_map!(recognize_float, parse_to!(i32)));
named!(pub num_f64(CompleteStr)-> f64, flat_map!(recognize_float, parse_to!(f64)));

#[test]
fn parse_i32() {
    let (_, res) = num_i32(CompleteStr("15")).unwrap();
    assert_eq!(res, 15);
}
#[test]
fn parse_i32_negative() {
    let (_, res) = num_i32(CompleteStr("-15")).unwrap();
    assert_eq!(res, -15);
}
