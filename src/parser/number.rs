use nom::number::complete::{ recognize_float};
use nom::character::complete::digit1;
named!(integer_str(&str) -> &str, recognize!(
    pair!(
        opt!(alt!(tag!("+") | tag!("-"))),
        digit1
)));
named!(pub num_u8(&str)-> u8, flat_map!(recognize_float, parse_to!(u8)));
named!(pub num_u32(&str)-> u32, flat_map!(recognize_float, parse_to!(u32)));
named!(pub num_i16(&str)-> i16, flat_map!(recognize_float, parse_to!(i16)));
named!(pub num_i32(&str)-> i32, flat_map!(recognize_float, parse_to!(i32)));
named!(pub num_f64(&str)-> f64, flat_map!(recognize_float, parse_to!(f64)));

#[test]
fn parse_i32() {
    let (_, res) = num_i32("15;").unwrap();
    assert_eq!(res, 15);
}
#[test]
fn parse_i32_negative() {
    let (_, res) = num_i32("-15;").unwrap();
    assert_eq!(res, -15);
}
