use value::{Level, Value};

pub enum Step {
    Raise(Level, Expression),
    Where(Expression),
}

pub enum Expression {
    Literal(Value),
}
