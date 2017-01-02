use value::{Level, Value};

pub enum Instruction {
    Halt,
    ConditionalJump(usize),

    Pop,

    Raise(Level),

    Literal(Value),
}
