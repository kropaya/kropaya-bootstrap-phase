use std;
use std::str;
use nom;
use nom::IResult;

use ast;

/*named!(ParseText<&str, ast::Literal>,
  chain!(
    re_match!("\"") ~
    text: map!(many0!(
                  re_match!("[^\\\"]|(\\([\\\"]))") // Do the version that captures a group
                  // preceded!(tag!("\\"), alt!(tag!("\\") | tag!("\""))))),
                  ),
            |chunks: Vec<&str>| {
              let mut lit: String = String::new();
              for chunk in chunks {
                lit.push_str(&chunk);
              }
              ast::Literal::Text(lit)
            }
          ) ~
    re_match!("\""),
    || text));*/


named!(ParseText<&str, ast::Literal>,
  chain!(
    re_find!("\\A\"") ~
    result: map!(re_capture!("\\A(?:([^\\\\\"]+)|(?:\\\\([\\\\\"])))*"),
         |chunks: Vec<&str>| {
                let mut lit: String = String::new();
                //for chunks in double_chunks {
                  let mut real_chunks = chunks.iter();
                  real_chunks.next(); // This skips us past the 0th item in the chunks, which is the whole match.
                  for chunk in real_chunks {
                    lit.push_str(&chunk);
                  }
                //}
                ast::Literal::Text(lit)
         }) ~
    re_find!("\\A\""),
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
       re_match!(r"\\(\\(\\([_+]+[_+:]*\\)?[a-zA-Z][a-zA-Z0-9_$!?%=-]*\\)\\|\\([~!@$%^*_=\'`/?×÷≠⧺⧻§∘•≢∨∪∩□⊃∈+-]+[~!@$%^*_=\'`/?×÷≠⧺⧻§∘•≢∨∪∩□⊃∈+-]*\\)\\|\\(\\[\\]\\)\\|…\\)\\(\^[+-]\\)?"));

named!(ParseLiteralLabel<&str, ast::Label>,
       chain!(
         re_match!(r"&") ~
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
  //parse_a_string("\"\\\\a\"", "\\a");
  //parse_a_string("\"1\\\\23453\\\\\\\\32345sd\\\"af234\"", "abc");
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
