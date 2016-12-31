use value::{Level, Value};

pub enum Instruction {
    Halt,
    ConditionalJump(usize),

    Raise(Level),

    Literal(Value),
}
