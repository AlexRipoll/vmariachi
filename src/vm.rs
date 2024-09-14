use crate::instruction::Opcode;

#[derive(Debug)]
pub struct VM {
    registers: [i32; 32],
    program: Vec<u8>,
    program_counter: usize,
}

impl VM {
    pub fn new() -> Self {
        Self {
            registers: [0; 32],
            program: Vec::new(),
            program_counter: 0,
        }
    }

    pub fn run(&mut self) {
        while let Some(_) = self.execute_instruction() {
            self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    fn execute_instruction(&mut self) -> Option<()> {
        if self.program_counter >= self.program.len() {
            return None;
        }

        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u16;
                self.registers[register] = number as i32;
            }
            Opcode::HLT => {
                println!("HTL encountered");
                return None;
            }
            _ => {
                println!("unrecognized opcode found! Terminating!");
                return None;
            }
        }

        Some(())
    }

    pub fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.program_counter]);
        self.program_counter += 1;

        opcode
    }

    fn next_8_bits(&mut self) -> u8 {
        let operand = self.program[self.program_counter];
        self.program_counter += 1;

        operand
    }

    fn next_16_bits(&mut self) -> u16 {
        let operand: u16 = ((self.program[self.program_counter] as u16) << 8)
            | (self.program[self.program_counter + 1] as u16);
        self.program_counter += 2;

        operand
    }
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Opcode::LOAD,
            5 => Opcode::HLT,
            _ => Opcode::IGL,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::vm::VM;

    #[test]
    fn test_new_vm() {
        let vm = VM::new();
        assert_eq!(vm.registers, [0; 32]);
    }

    #[test]
    fn test_opcode_hlt() {
        let mut vm = VM::new();
        vm.program = vec![5, 0, 0, 0];
        vm.run_once();
        assert_eq!(vm.program_counter, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut vm = VM::new();
        vm.program = vec![255, 0, 0, 0];
        vm.run_once();
        assert_eq!(vm.program_counter, 1);
    }

    #[test]
    fn test_opcode_load() {
        let mut vm = VM::new();
        // [opcode, register, operand, operand]
        vm.program = vec![0, 0, 1, 244];
        vm.run_once();
        assert_eq!(vm.registers[0], 500);
    }
}
