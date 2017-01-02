use ast::{Expression, Step};
use bytecode::{Chunk, Instruction};
use std::cmp::max;

pub struct Codegen {
    pub chunk: Chunk,
    current_operand_stack_capacity: isize,
}

impl Codegen {
    pub fn new() -> Self {
        Codegen{
            chunk: Chunk{
                instructions: vec![],
                operand_stack_capacity: 0,
            },
            current_operand_stack_capacity: 0,
        }
    }

    pub fn codegen_step(&mut self, step: &Step) {
        match *step {
            Step::Perform(ref expression) => {
                self.codegen_expression(expression);

                self.chunk.instructions.push(Instruction::Pop);
                self.update_current_operand_stack_capacity(-1);
            },
            Step::Raise(level, ref message) => {
                self.codegen_expression(message.clone());

                self.chunk.instructions.push(Instruction::Raise(level));
                self.update_current_operand_stack_capacity(-1);
            },
            Step::Where(ref condition) => {
                self.codegen_expression(condition);

                let jump_target = self.chunk.instructions.len() + 2;
                self.chunk.instructions.push(Instruction::ConditionalJump(jump_target));
                self.update_current_operand_stack_capacity(-1);

                self.chunk.instructions.push(Instruction::Halt);
            },
        }
    }

    pub fn codegen_expression(&mut self, expression: &Expression) {
        match *expression {
            Expression::Literal(ref value) => {
                self.chunk.instructions.push(Instruction::Literal(value.clone()));
                self.update_current_operand_stack_capacity(1);
            },
        }
    }

    fn update_current_operand_stack_capacity(&mut self, delta: isize) {
        self.current_operand_stack_capacity += delta;
        self.chunk.operand_stack_capacity = max(
            self.current_operand_stack_capacity as usize,
            self.chunk.operand_stack_capacity,
        );
    }
}

#[cfg(test)]
mod tests {
    use ast::{Expression, Step};
    use bytecode::{Chunk, Instruction};
    use super::*;
    use value::Value;

    #[test]
    fn test_codegen() {
        let mut codegen = Codegen::new();
        codegen.codegen_step(&Step::Where(Expression::Literal(Value::Boolean(false))));
        codegen.codegen_step(&Step::Perform(Expression::Literal(Value::Boolean(true))));
        assert_eq!(
            codegen.chunk,
            Chunk{
                instructions: vec![
                    Instruction::Literal(Value::Boolean(false)),
                    Instruction::ConditionalJump(3),
                    Instruction::Halt,
                    Instruction::Literal(Value::Boolean(true)),
                    Instruction::Pop,
                ],
                operand_stack_capacity: 1,
            }
        );
    }
}
