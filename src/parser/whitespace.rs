use nom::*;

named!(pub space<&str, ()>, value!((), one_of!(" \t\n")));
named!(pub comment(&str) -> (), value!((), delimited!(tag!(";"), not_line_ending, line_ending)));
named!(pub space_or_comment(&str) -> (), alt!(space | comment));

#[test]
fn test_comment() {
  assert_eq!(
    comment("; Function            function_0\n\0",).unwrap().0,
    "\0"
  );
}
#[test]
fn test_space_comment() {
  assert_eq!(
    comment(
      ";   [0] R2 := G[\"math\"]
GETTABLE  R2 R2 K1      ;   [1] R2 := R2[\"random\"]\0",
    )
    .unwrap()
    .0,
    "GETTABLE  R2 R2 K1      ;   [1] R2 := R2[\"random\"]\0"
  );
}

/*
#[macro_export]
macro_rules! sp (
  ($i:expr, $($args:tt)*) => (
    {
      use nom::Convert;
      use nom::Err;
      use $crate::parser::whitespace::space;

      match sep!($i, space, $($args)*) {
        Err(e) => Err(e),
        Ok((i1,o))    => {
          match space(i1) {
            Err(e) => Err(Err::convert(e)),
            Ok((i2,_))    => Ok((i2, o))
          }
        }
      }
    }
  )
);
#[macro_export]
macro_rules! spc (
  ($i:expr, $($args:tt)*) => (
    {
      use nom::Convert;
      use nom::Err;
      use $crate::parser::whitespace::space_or_comment;

      match sep!($i, space_or_comment, $($args)*) {
        Err(e) => Err(e),
        Ok((i1,o))    => {
          match space_or_comment(i1) {
            Err(e) => Err(Err::convert(e)),
            Ok((i2,_))    => Ok((i2, o))
          }
        }
      }
    }
  )
);
*/
