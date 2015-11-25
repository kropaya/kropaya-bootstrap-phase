pub enum Variable {
  Plain(String)
}
pub enum Label {
  Literal(String),
  Variable(Variable)
}
pub enum TVN {
  Type(String),
  Value(Box<Literal>),
  Nothing
}
pub struct SingularRow {
  label: Label,
  extra: TVN
}
pub enum Restriction {
  Variable(Variable),
  SingularRow(Box<SingularRow>)
}
pub struct Product {
  fixed_rows: Vec<Box<SingularRow>>,
  polymorphic_extension: Vec<Variable>,
  restrictions: Vec<Restriction>
}
pub enum Literal {
  Text(String),
  Integer(i64),
  Float(f64),
  //Product(Product),
  //Sum(Sum),
  SingularRow(Box<SingularRow>),
  Label(Label),
  Underscore
}
