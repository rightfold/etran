use ast::{Expression, Step};
use bytecode::{Chunk, Instruction};
use std::cmp::max;
use std::collections::HashMap;

pub struct Codegen<'str> {
    pub chunk: Chunk,
    current_operand_stack_capacity: isize,
    local_variable_slots: HashMap<&'str str, usize>,
}

impl<'str> Codegen<'str> {
    pub fn new() -> Self {
        Codegen{
            chunk: Chunk{
                instructions: vec![],
                operand_stack_capacity: 0,
                local_variable_count: 0,
            },
            current_operand_stack_capacity: 0,
            local_variable_slots: HashMap::new(),
        }
    }

    pub fn codegen_step(&mut self, step: &Step<'str>) {
        match *step {
            Step::Perform(ref expression) => {
                self.codegen_expression(expression);

                self.chunk.instructions.push(Instruction::Pop);
                self.update_current_operand_stack_capacity(-1);
            },
            Step::PerformAs(ref expression, name) => {
                self.codegen_expression(expression);

                let slot = self.new_local_variable(name);
                self.chunk.instructions.push(Instruction::Store(slot));
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

    fn new_local_variable(&mut self, name: &'str str) -> usize {
        let slot = self.chunk.local_variable_count;
        self.chunk.local_variable_count += 1;
        self.local_variable_slots.insert(name, slot);
        slot
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
        codegen.codegen_step(&Step::PerformAs(Expression::Literal(Value::Boolean(false)), "x"));
        codegen.codegen_step(&Step::PerformAs(Expression::Literal(Value::Boolean(false)), "x"));
        assert_eq!(
            codegen.chunk,
            Chunk{
                instructions: vec![
                    Instruction::Literal(Value::Boolean(false)),
                    Instruction::ConditionalJump(3),
                    Instruction::Halt,
                    Instruction::Literal(Value::Boolean(true)),
                    Instruction::Pop,
                    Instruction::Literal(Value::Boolean(false)),
                    Instruction::Store(0),
                    Instruction::Literal(Value::Boolean(false)),
                    Instruction::Store(1),
                ],
                operand_stack_capacity: 1,
                local_variable_count: 2,
            }
        );
    }
}
