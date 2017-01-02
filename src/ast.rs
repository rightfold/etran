use value::{Level, Value};

#[derive(Debug, Eq, PartialEq)]
pub enum Definition<'str> {
    Channel(&'str str, Type, &'str str),
    Filter(&'str str, From<'str>, Vec<Step<'str>>),
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
pub enum Step<'str> {
    Perform(Expression<'str>),
    PerformAs(Expression<'str>, &'str str),
    Raise(Level, Expression<'str>),
    Where(Expression<'str>),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Expression<'str> {
    Variable(&'str str),
    Literal(Value),
}
