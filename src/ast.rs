use value::{Level, Value};

#[derive(Debug, Eq, PartialEq)]
pub enum Definition<'str> {
    Channel(&'str str, Type, &'str str),
    Filter(&'str str, From<'str>, Vec<Step>),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Type {
    Real,
}

#[derive(Debug, Eq, PartialEq)]
pub struct From<'str> {
    pub stream: &'str str,
    pub name: &'str str,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Step {
    Raise(Level, Expression),
    Where(Expression),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Expression {
    Literal(Value),
}
