use std;
use std::str;
use nom;
use nom::IResult;

use ast;

named!(ParseText<&str, ast::Literal>,
  chain!(
    tag_s!("\"") ~
    result: map!(many0!(alt!(
          is_not_s!("\\\"") |
          chain!(
            tag_s!("\\") ~
            character: alt!(tag_s!("\\") | tag_s!("\"")),
            || character))),
          |chunks: Vec<&str>| {
                let mut lit: String = String::new();
                for chunk in chunks.iter() {
                  lit.push_str(&chunk);
                }
                ast::Literal::Text(lit)
          }) ~
    tag_s!("\""),
    || result));

named!(ParseInteger<&[u8], ast::Literal>,
       chain!(
         sign:   alt!(tag!("+") | tag!("-"))? ~
         digits: call!(nom::digit),
         || {
           let base = String::from_utf8_lossy(digits).parse::<i64>().unwrap_or(0);
           let result = match sign {
             Some(b"-") => base * -1,
             _          => base
           };
           ast::Literal::Integer(result)
         }));

named!(valid_variable<&str, &str>,
       re_find!(r"\A((([_+]+[_+:]*)?[a-zA-Z][a-zA-Z0-9_$!?%=-]*)|([~!@$%^*_='`?×÷≠⧺⧻§∘•≢∨∪∩□⊃∈+-]+[~!@$%^*_='`?×÷≠⧺⧻§∘•≢∨∪∩□⊃∈+-]*)|([\\])|…)(\^[+-])?"));

named!(ParseLiteralLabel<&str, ast::Label>,
       chain!(
         tag_s!(r"&") ~
         name: valid_variable,
         || ast::Label::Literal(name.to_string())));

//named!(ParseSingularRow<&[u8], ast::SingularRow>,
//       chain!(
//         label: alt!(ParseLiteralLabel | 

#[test]
fn parse_string_test() {
  fn parse_a_string(foo1: &str, foo2: &str) {
    let v = foo1;
    match ParseText(v) {
      nom::IResult::Done(_, res) => {
        assert_eq!(res, ast::Literal::Text(foo2.to_string()));
      }
       err => {
        println!("Parsing string: {}, to parse as {}.", foo1, foo2);
        println!("{:?}", err);
        assert!(false);
      }
    }
  }
  parse_a_string("\"\"", "");
  parse_a_string("\"abc\"", "abc");
  parse_a_string("\"\\\"\"", "\"");
  parse_a_string("\"\\\\\"", "\\");
  parse_a_string("\"a\\\\\"", "a\\");
  parse_a_string("\"\\\\a\"", "\\a");
  parse_a_string("\"1\\\\23453\\\\\\\\32345sd\\\"af234\"", "1\\23453\\\\32345sd\"af234");
}

#[test]
fn parse_integer_test() {
  fn parse_an_integer(foo1: &str, foo2: i64) {
    let v = foo1.as_bytes();
    match ParseInteger(v) {
      nom::IResult::Done(_, res) => {
        assert_eq!(res, ast::Literal::Integer(foo2));
      }
      err => {
        //println!("{:?}", err);
        assert!(false);
      }
    }
  }
  parse_an_integer("0", 0);
  parse_an_integer("+0", 0);
  parse_an_integer("-0", 0);
  parse_an_integer("6", 6);
  parse_an_integer("+4", 4);
  parse_an_integer("-1000000", -1000000);
}

#[test]
fn recognise_a_variable_test() {
  assert_eq!(valid_variable("a"), nom::IResult::Done("", "a"));
  assert_eq!(valid_variable("a "), nom::IResult::Done(" ", "a"));
  assert_eq!(valid_variable("__:a^+- "), nom::IResult::Done("- ", "__:a^+"));
}

#[test]
fn parse_a_literal_label_test() {
  assert_eq!(ParseLiteralLabel("&foobar"), nom::IResult::Done("", ast::Label::Literal("foobar".to_string())));
}
