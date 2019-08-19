use super::ParseResult;
use nom::combinator::*;
use nom::number::complete::recognize_float;

macro_rules! make_num_parser {
    ($n:ident, $t:ty) => {
        pub fn $n(input: &str) -> ParseResult<$t> {
            map_res(recognize_float, |s: &str| s.parse::<$t>())(input)
        }
    };
}

make_num_parser!(num_u8, u8);
// make_num_parser!(num_u32, u32);
make_num_parser!(num_i16, i16);
// make_num_parser!(num_i32, i32);
make_num_parser!(num_f64, f64);

#[test]
fn parse_i16() {
    let (_, res) = num_i16("15;").unwrap();
    assert_eq!(res, 15);
}
#[test]
fn parse_i16_negative() {
    let (_, res) = num_i16("-15;").unwrap();
    assert_eq!(res, -15);
}
