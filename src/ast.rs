#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Variable {
  Plain(String)
}
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Label {
  Literal(String),
  Variable(Variable)
}
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum TVN {
  Type(String),
  Value(Box<Literal>),
  Nothing
}
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct SingularRow {
  label: Label,
  extra: TVN
}
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Restriction {
  Variable(Variable),
  SingularRow(Box<SingularRow>)
}
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Product {
  fixed_rows: Vec<Box<SingularRow>>,
  polymorphic_extension: Vec<Variable>,
  restrictions: Vec<Restriction>
}
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Literal {
  Text(String),
  Integer(i64),
  //Float(f64),
  //Product(Product),
  //Sum(Sum),
  SingularRow(Box<SingularRow>),
  Label(Label),
  Underscore
}
