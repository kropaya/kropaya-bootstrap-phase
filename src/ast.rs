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
  pub label: Label,
  pub extra: TVN
}
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Restriction {
  Variable(Variable),
  SingularRow(Box<SingularRow>)
}
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Row {
  fixed_rows: Vec<Box<SingularRow>>,
  polymorphic_extension: Vec<Variable>,
  restrictions: Vec<Restriction>
}
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Literal {
  Text(String),
  Integer(i64),
  //Float(f64),
  Product(Row),
  Sum(Row),
  SingularRow(Box<SingularRow>),
  Label(Label),
  Underscore
}
