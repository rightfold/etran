use value::{Level, Value};

#[derive(Debug, Eq, PartialEq)]
pub struct Chunk {
    pub instructions: Vec<Instruction>,
    pub operand_stack_capacity: usize,
    pub local_variable_count: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
    Halt,
    ConditionalJump(usize),

    Pop,

    Store(usize),

    Raise(Level),

    Literal(Value),
}
