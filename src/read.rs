use std::str;
use nom;
use nom::IResult;

use ast;

named!(ParseText<&[u8], ast::Literal>,
  delimited!(
    tag!("\""),
      map!(many0!(alt!(
                  is_not!([92, 34]) | 
                  preceded!(tag!("\\"), alt!(tag!("\\") | tag!("\""))))),
           |chunks: Vec<&[u8]>| ast::Literal::Text(
               String::from_utf8(chunks.into_iter().flat_map(
                       |byte_array| byte_array.into_iter().cloned())
                   .collect())
               .unwrap())
      ),
    tag!("\"")));


#[test]
fn parse_string_test() {
  fn parse_a_string(foo1: &str, foo2: &str) {
    let v = foo1.as_bytes();
    match ParseText(v) {
      nom::IResult::Done(_, res) => {
        assert_eq!(res, ast::Literal::Text(foo2.to_string()));
      }
      _ => {
        assert!(false);
      }
    }
  }
  parse_a_string("\"abc\"", "abc");
  parse_a_string("\"\"", "");
  parse_a_string("\"\\\"\"", "\"");
}
