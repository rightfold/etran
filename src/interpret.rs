use bytecode::Instruction;
use value::{Level, Value};

pub fn interpret<Raise>(instructions: &[Instruction], mut raise: Raise)
    where Raise: FnMut(Level, &Value) {
    let mut program_counter = 0;
    let mut operand_stack = vec![];
    loop {
        match instructions[program_counter] {
            Instruction::Halt => return,
            Instruction::ConditionalJump(jump_target) => {
                let condition = operand_stack.pop().unwrap();
                match condition {
                    Value::Boolean(true) => program_counter = jump_target,
                    _ => program_counter += 1,
                }
            },

            Instruction::Raise(level) => {
                let message = operand_stack.pop().unwrap();
                raise(level, &message);
                program_counter += 1;
            },

            Instruction::Literal(ref value) => {
                operand_stack.push(value.clone());
                program_counter += 1;
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use bytecode::Instruction;
    use super::*;
    use value::{Level, Value};

    #[test]
    fn test_interpret() {
        let mut level = None;
        let mut message = None;
        interpret(&[
            Instruction::Literal(Value::Boolean(true)),
            Instruction::Raise(Level::Info),
            Instruction::Halt,
        ], |l, m| { level = Some(l); message = Some(m.clone()); });
        assert_eq!(level, Some(Level::Info));
        assert_eq!(message, Some(Value::Boolean(true)));
    }
}
