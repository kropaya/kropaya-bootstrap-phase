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

named!(ParseVariable<&str, ast::Variable>,
       map!(valid_variable,
            |x: &str| ast::Variable::Plain(x.to_string())));

named!(ParseLiteralLabel<&str, ast::Label>,
       chain!(
         tag_s!(r"&") ~
         name: valid_variable,
         || ast::Label::Literal(name.to_string())));

named!(ws<&str, &str>,
       chain!(
         many1!(tag_s!(" ")),
         || ""));

fn ret_nothing(i:&str) -> IResult<&str, ast::TVN> { nom::IResult::Done(i,ast::TVN::Nothing) }

named!(ParseSingularRow<&str, ast::SingularRow>,
       chain!(
         label: alt!(ParseLiteralLabel | map!(ParseVariable, |var: ast::Variable| ast::Label::Variable(var))) ~
         ws? ~
         extra: alt!(
           /*chain!(
             tag_s!(":") ~
             ws? ~
             type_ascription: ParseType,
             || ast::TVN::Type(type_ascription)) |
           chain!(
             tag_s!("⇒") ~
             ws? ~
             value: ParseLiteral,
             || ast::TVN::Value(value)) |*/
           ret_nothing),
         || ast::SingularRow { label: label, extra: extra }));

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
  assert_eq!(ParseLiteralLabel("&%$%"), nom::IResult::Done("", ast::Label::Literal("%$%".to_string())));
}

#[test]
fn parse_a_variable_test() {
  assert_eq!(ParseVariable("foobar"), nom::IResult::Done("", ast::Variable::Plain("foobar".to_string())));
  assert_eq!(ParseVariable("…"), nom::IResult::Done("", ast::Variable::Plain("…".to_string())));
}

#[test]
fn parse_a_singular_row() {
  assert_eq!(ParseSingularRow("&foo "), nom::IResult::Done("", ast::SingularRow { label: ast::Label::Literal("foo".to_string()), extra: ast::TVN::Nothing }));
}
