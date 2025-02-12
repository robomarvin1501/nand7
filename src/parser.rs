use core::panic;

use phf;

use crate::instructions::{ArithmeticType, BinaryArithmeticOperator, Instruction, Pop, Push, Segment, ShiftArithmeticOperator, UnaryArithmeticOperator};

// add, sub, neg, and, or, not, shiftleft, shiftright, eq, gt, lt

const COMMENT_BEGIN: &str = "//";

pub const OPERANDS_MEMORY: [&'static str; 2] = ["push", "pop"];

pub const OPERANDS_ARITHMETIC_IMPLICIT: phf::Map<&'static str, Instruction> = phf::phf_map! {
    "add" => Instruction::CArithmetic(ArithmeticType::Binary(BinaryArithmeticOperator::Add)),
    "sub" => Instruction::CArithmetic(ArithmeticType::Binary(BinaryArithmeticOperator::Subtract)),
    "neg" => Instruction::CArithmetic(ArithmeticType::Unary(UnaryArithmeticOperator::Negate)),
    "and" => Instruction::CArithmetic(ArithmeticType::Binary(BinaryArithmeticOperator::And)),
    "or" => Instruction::CArithmetic(ArithmeticType::Binary(BinaryArithmeticOperator::Or)),
    "not" => Instruction::CArithmetic(ArithmeticType::Unary(UnaryArithmeticOperator::Not)),
    "shiftleft" => Instruction::CArithmetic(ArithmeticType::Shift(ShiftArithmeticOperator::ShiftLeft)),
    "shiftright" => Instruction::CArithmetic(ArithmeticType::Shift(ShiftArithmeticOperator::ShiftRight)),
    "eq" => Instruction::CArithmetic(ArithmeticType::Binary(BinaryArithmeticOperator::Eq)),
    "gt" => Instruction::CArithmetic(ArithmeticType::Binary(BinaryArithmeticOperator::Gt)),
    "lt" => Instruction::CArithmetic(ArithmeticType::Binary(BinaryArithmeticOperator::Lt))
};

pub fn parse(lines: Vec<String>) -> Vec<Instruction> {
    let whitespace_cleaned_lines = clear_whitespace(lines);

    let mut parsed_lines: Vec<Instruction> = vec![];
    for line in whitespace_cleaned_lines {
        match OPERANDS_ARITHMETIC_IMPLICIT.get(&line) {
            Some(instr) => {
                parsed_lines.push(instr.to_owned());
                continue;
            }
            None => {}
        }
        match operand_memory(&line) {
            Some(instruction) => parsed_lines.push(instruction),
            None => {}
        }
    }
    parsed_lines
}

fn clear_whitespace(lines: Vec<String>) -> Vec<String> {
    let mut whitespace_cleaned_lines: Vec<String> = vec![];
    for line in lines {
        if line.is_empty() || line.starts_with(COMMENT_BEGIN) {
        } else if let Some(comment_index) = line.find(COMMENT_BEGIN) {
            let trimmed = &line[..comment_index].trim();
            whitespace_cleaned_lines.push(trimmed.to_string());
        } else {
            whitespace_cleaned_lines.push(line);
        }
    }
    whitespace_cleaned_lines
}

fn operand_memory(line: &String) -> Option<Instruction> {
    for operand in OPERANDS_MEMORY {
        if !line.starts_with(operand) {
            continue;
        }
        let mut line_details = line.split_whitespace();
        line_details.next();
        let unparsed_segment = line_details.next().unwrap();
        let unparsed_index = line_details.next().unwrap();

        let segment: Segment = Segment::from(unparsed_segment);
        let index: u16 = unparsed_index.parse().unwrap();
        match operand {
            "push" => {
                return Some(Instruction::CPush(Push::new(segment, index)));
            }
            "pop" => {
                return Some(Instruction::CPop(Pop::new(segment, index)));
            }
            _ => panic!("Undefined operand: {operand}"),
        }
    }
    None
}
