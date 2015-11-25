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
