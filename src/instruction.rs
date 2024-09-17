#[derive(Debug, PartialEq, Clone)]
pub enum Opcode {
    LOAD, // LOAD
    ADD,  // ADD
    SUB,  // SUBTRACT
    MUL,  // MULTIPLY
    DIV,  // DIVIDE
    HLT,  // HALT
    JMP,  // JUMP (ABSOLUTE)
    JMPF, // JUMP FORWARD (RELATIVE)
    JMPB, // JUMP BACKWARD (RELATIVE)
    EQ,   // EQUAL
    NEQ,  // NOT EQUAL
    GT,   // GREATER THAN
    LT,   // LESS THAN
    GTE,  // GREATER THAN OR EQUAL
    LTE,  // LESS THAN OR EQUAL
    JEQ,  // JUMP IF EQUAL
    JNEQ, // JUMP IF NOT EQUAL
    IGL,  // ILLEGAL
}

#[derive(Debug)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Self {
        Self { opcode }
    }
}

impl From<&str> for Opcode {
    fn from(v: &str) -> Self {
        match v {
            "load" => Opcode::LOAD,
            "add" => Opcode::ADD,
            "sub" => Opcode::SUB,
            "mul" => Opcode::MUL,
            "div" => Opcode::DIV,
            "hlt" => Opcode::HLT,
            "jmp" => Opcode::JMP,
            "jmpf" => Opcode::JMPF,
            "jmpb" => Opcode::JMPB,
            "eq" => Opcode::EQ,
            "neq" => Opcode::NEQ,
            "gt" => Opcode::GT,
            "lt" => Opcode::LT,
            "gte" => Opcode::GTE,
            "lte" => Opcode::LTE,
            "jeq" => Opcode::JEQ,
            "jneq" => Opcode::JNEQ,
            _ => Opcode::IGL,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::instruction::{Instruction, Opcode};

    #[test]
    fn test_new_opcode() {
        let opcode = Opcode::HLT;
        let instruction = Instruction::new(opcode);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}
