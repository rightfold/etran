use value::{Level, Value};

#[derive(Debug, Eq, PartialEq)]
pub enum Step {
    Raise(Level, Expression),
    Where(Expression),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Expression {
    Literal(Value),
}
