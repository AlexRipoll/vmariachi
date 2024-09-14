#[derive(Debug, PartialEq)]
pub enum Opcode {
    LOAD, // LOAD
    ADD,  // ADD
    SUB,  // SUBTRACT
    MUL,  // MULTIPLY
    DIV,  // DIVIDE
    HLT,  // HALT
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
