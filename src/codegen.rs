use ast::{Expression, Step};
use bytecode::Instruction;

pub fn codegen_step(instructions: &mut Vec<Instruction>, step: &Step) {
    match *step {
        Step::Perform(ref expression) => {
            codegen_expression(instructions, expression);
            instructions.push(Instruction::Pop);
        },
        Step::Raise(level, ref message) => {
            codegen_expression(instructions, message.clone());
            instructions.push(Instruction::Raise(level));
        },
        Step::Where(ref condition) => {
            codegen_expression(instructions, condition);
            let jump_target = instructions.len() + 1;
            instructions.push(Instruction::ConditionalJump(jump_target));
            instructions.push(Instruction::Halt);
        },
    }
}

pub fn codegen_expression(instructions: &mut Vec<Instruction>, expression: &Expression) {
    match *expression {
        Expression::Literal(ref value) =>
            instructions.push(Instruction::Literal(value.clone())),
    }
}
