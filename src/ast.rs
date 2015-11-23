enum Variable {
  Plain(String)
}
enum Label {
  Literal(String),
  Variable(Variable)
}
enum TVN {
  Type(String),
  Value(Box<Literal>),
  Nothing
}
struct SingularRow {
  label: Label,
  extra: TVN
}
enum Restriction {
  Variable(Variable),
  SingularRow(Box<SingularRow>)
}
struct Product {
  fixed_rows: Vec<Box<SingularRow>>,
  polymorphic_extension: Vec<Variable>,
  restrictions: Vec<Restriction>
}
enum Literal {
  Text(String),
  Integer(i64),
  Float(f64),
  //Product(Product),
  //Sum(Sum),
  SingularRow(Box<SingularRow>),
  Label(Label),
  Underscore
}
