use std::usize;

use crate::instruction::Opcode;

#[derive(Debug)]
pub struct VM {
    pub registers: [i32; 32],
    pub program: Vec<u8>,
    program_counter: usize,
    remainder: u32,
    equal_flag: bool,
}

impl VM {
    pub fn new() -> Self {
        Self {
            registers: [0; 32],
            program: Vec::new(),
            program_counter: 0,
            remainder: 0,
            equal_flag: false,
        }
    }

    pub fn run(&mut self) {
        while let Some(_) = self.execute_instruction() {
            self.execute_instruction();
        }
    }

    pub fn program(&self) -> &Vec<u8> {
        &self.program
    }

    pub fn registers(&self) -> &[i32; 32] {
        &self.registers
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
                let register_idx = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u16;
                self.registers[register_idx] = number as i32;
            }
            Opcode::ADD => {
                let first_register = self.registers[self.next_8_bits() as usize];
                let second_register = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = first_register + second_register;
            }
            Opcode::SUB => {
                let first_register = self.registers[self.next_8_bits() as usize];
                let second_register = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = first_register - second_register;
            }
            Opcode::MUL => {
                let first_register = self.registers[self.next_8_bits() as usize];
                let second_register = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = first_register * second_register;
            }
            Opcode::DIV => {
                let first_register = self.registers[self.next_8_bits() as usize];
                let second_register = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = first_register / second_register;
                // TODO: handle division by 0
                self.remainder = (first_register % second_register) as u32;
            }
            Opcode::HLT => {
                println!("HTL encountered");
                return None;
            }
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.program_counter = target as usize;
            }
            Opcode::JMPF => {
                let jumps = self.registers[self.next_8_bits() as usize];
                self.program_counter += jumps as usize;
            }
            Opcode::JMPB => {
                let jumps = self.registers[self.next_8_bits() as usize];
                self.program_counter -= jumps as usize;
            }
            Opcode::EQ => {
                let first_value = self.registers[self.next_8_bits() as usize];
                let second_value = self.registers[self.next_8_bits() as usize];
                self.equal_flag = first_value == second_value;
                self.next_8_bits();
            }
            Opcode::NEQ => {
                let first_value = self.registers[self.next_8_bits() as usize];
                let second_value = self.registers[self.next_8_bits() as usize];
                self.equal_flag = first_value != second_value;
                self.next_8_bits();
            }
            Opcode::GT => {
                let first_value = self.registers[self.next_8_bits() as usize];
                let second_value = self.registers[self.next_8_bits() as usize];
                self.equal_flag = first_value > second_value;
                self.next_8_bits();
            }
            Opcode::LT => {
                let first_value = self.registers[self.next_8_bits() as usize];
                let second_value = self.registers[self.next_8_bits() as usize];
                self.equal_flag = first_value < second_value;
                self.next_8_bits();
            }
            Opcode::GTQ => {
                let first_value = self.registers[self.next_8_bits() as usize];
                let second_value = self.registers[self.next_8_bits() as usize];
                self.equal_flag = first_value >= second_value;
                self.next_8_bits();
            }
            Opcode::LTQ => {
                let first_value = self.registers[self.next_8_bits() as usize];
                let second_value = self.registers[self.next_8_bits() as usize];
                self.equal_flag = first_value <= second_value;
                self.next_8_bits();
            }
            Opcode::JEQ => {
                let target = self.registers[self.next_8_bits() as usize];
                if self.equal_flag {
                    self.program_counter = target as usize;
                }
            }
            Opcode::JNEQ => {
                let target = self.registers[self.next_8_bits() as usize];
                if !self.equal_flag {
                    self.program_counter = target as usize;
                }
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
            1 => Opcode::ADD,
            2 => Opcode::SUB,
            3 => Opcode::MUL,
            4 => Opcode::DIV,
            5 => Opcode::HLT,
            6 => Opcode::JMP,
            7 => Opcode::JMPF,
            8 => Opcode::JMPB,
            9 => Opcode::EQ,
            10 => Opcode::NEQ,
            11 => Opcode::GT,
            12 => Opcode::LT,
            13 => Opcode::GTQ,
            14 => Opcode::LTQ,
            15 => Opcode::JEQ,
            16 => Opcode::JNEQ,
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

    #[test]
    fn test_opcode_add() {
        let mut vm = VM::new();
        // [opcode, register, operand, operand]
        vm.program = vec![0, 0, 1, 244]; // LOAD $0 #500
        vm.program.extend_from_slice(&vec![0, 1, 0, 7]); // LOAD $1 #7
        vm.program.extend_from_slice(&vec![1, 0, 1, 2]); // ADD $0 $1 $2 (ADD  registers 0 and 1 and set result to register 2)
        vm.run();
        assert_eq!(vm.registers[2], 507);
    }

    #[test]
    fn test_opcode_sub() {
        let mut vm = VM::new();
        // [opcode, register, operand, operand]
        vm.program = vec![0, 0, 1, 244]; // LOAD $0 #500
        vm.program.extend_from_slice(&vec![0, 1, 0, 7]); // LOAD $1 #7
        vm.program.extend_from_slice(&vec![2, 0, 1, 2]); // SUB $0 $1 $2 (ADD  registers 0 and 1 and set result to register 2)
        vm.run();
        assert_eq!(vm.registers[2], 493);
    }

    #[test]
    fn test_opcode_mul() {
        let mut vm = VM::new();
        // [opcode, register, operand, operand]
        vm.program = vec![0, 0, 1, 244]; // LOAD $0 #500
        vm.program.extend_from_slice(&vec![0, 1, 0, 7]); // LOAD $1 #7
        vm.program.extend_from_slice(&vec![3, 0, 1, 2]); // MUL $0 $1 $2 (ADD  registers 0 and 1 and set result to register 2)
        vm.run();
        assert_eq!(vm.registers[2], 3500);
    }

    #[test]
    fn test_opcode_div_without_remainder() {
        let mut vm = VM::new();
        // [opcode, register, operand, operand]
        vm.program = vec![0, 0, 1, 244]; // LOAD $0 #500
        vm.program.extend_from_slice(&vec![0, 1, 0, 5]); // LOAD $1 #5
        vm.program.extend_from_slice(&vec![4, 0, 1, 2]); // MUL $0 $1 $2 (ADD  registers 0 and 1 and set result to register 2)
        vm.run();
        assert_eq!(vm.registers[2], 100);
        assert_eq!(vm.remainder, 0);
    }

    #[test]
    fn test_opcode_div_with_remainder() {
        let mut vm = VM::new();
        // [opcode, register, operand, operand]
        vm.program = vec![0, 0, 1, 244]; // LOAD $0 #500
        vm.program.extend_from_slice(&vec![0, 1, 0, 6]); // LOAD $1 #6
        vm.program.extend_from_slice(&vec![4, 0, 1, 2]); // MUL $0 $1 $2 (ADD  registers 0 and 1 and set result to register 2)
        vm.run();
        assert_eq!(vm.registers[2], 83);
        assert_eq!(vm.remainder, 2);
    }

    #[test]
    fn test_opcode_jmp() {
        let mut vm = VM::new();
        // [opcode, register, operand, operand]
        vm.registers[2] = 7;
        vm.program = vec![6, 2, 0, 0]; // JMP $1 (JMP to Opcode at program[idx] where idx is the value stored at register 2)
        vm.run_once();
        assert_eq!(vm.program_counter, 7);
    }

    #[test]
    fn test_opcode_jmpf() {
        let mut vm = VM::new();
        // [opcode, register, operand, operand]
        vm.registers[2] = 2;
        vm.program = vec![7, 2, 0, 0, 0, 0, 1, 124]; // JMP $1 (JMP to Opcode at program[idx] where idx is the value stored at register 2)
        vm.run_once();
        assert_eq!(vm.program_counter, 4);
    }

    #[test]
    fn test_opcode_jmpb() {
        let mut vm = VM::new();
        // [opcode, register, operand, operand]
        vm.registers[2] = 2;
        vm.program = vec![8, 2, 0, 0, 0, 0, 1, 124]; // JMP $1 (JMP to Opcode at program[idx] where idx is the value stored at register 2)
        vm.run_once();
        assert_eq!(vm.program_counter, 0);
    }

    #[test]
    fn test_opcode_eq_true() {
        let mut vm = VM::new();
        vm.registers[0] = 2;
        vm.registers[1] = 2;
        vm.program = vec![9, 0, 1, 0]; // EQ $0 $1
        vm.run_once();
        assert!(vm.equal_flag);
    }

    #[test]
    fn test_opcode_eq_false() {
        let mut vm = VM::new();
        vm.registers[0] = 2;
        vm.registers[1] = 5;
        vm.program = vec![9, 0, 1, 0]; // EQ $0 $1
        vm.run_once();
        assert!(!vm.equal_flag);
    }

    #[test]
    fn test_opcode_neq_true() {
        let mut vm = VM::new();
        vm.registers[0] = 1;
        vm.registers[1] = 6;
        vm.program = vec![10, 0, 1, 0]; // NEQ $0 $1
        vm.run_once();
        assert!(vm.equal_flag);
    }

    #[test]
    fn test_opcode_neq_false() {
        let mut vm = VM::new();
        vm.registers[0] = 2;
        vm.registers[1] = 2;
        vm.program = vec![10, 0, 1, 0]; // NEQ $0 $1
        vm.run_once();
        assert!(!vm.equal_flag);
    }

    #[test]
    fn test_opcode_gt_true() {
        let mut vm = VM::new();
        vm.registers[0] = 6;
        vm.registers[1] = 5;
        vm.program = vec![11, 0, 1, 0]; // GT $0 $1
        vm.run_once();
        assert!(vm.equal_flag);
    }

    #[test]
    fn test_opcode_gt_false() {
        let mut vm = VM::new();
        vm.registers[0] = 2;
        vm.registers[1] = 2;
        vm.program = vec![11, 0, 1, 0]; // GT $0 $1
        vm.run_once();
        assert!(!vm.equal_flag);
    }

    #[test]
    fn test_opcode_lt_true() {
        let mut vm = VM::new();
        vm.registers[0] = 5;
        vm.registers[1] = 6;
        vm.program = vec![12, 0, 1, 0]; // LT $0 $1
        vm.run_once();
        assert!(vm.equal_flag);
    }

    #[test]
    fn test_opcode_lt_false() {
        let mut vm = VM::new();
        vm.registers[0] = 2;
        vm.registers[1] = 2;
        vm.program = vec![12, 0, 1, 0]; // LT $0 $1
        vm.run_once();
        assert!(!vm.equal_flag);
    }

    #[test]
    fn test_opcode_gtq_greater_true() {
        let mut vm = VM::new();
        vm.registers[0] = 6;
        vm.registers[1] = 5;
        vm.program = vec![13, 0, 1, 0]; // GTQ $0 $1
        vm.run_once();
        assert!(vm.equal_flag);
    }

    #[test]
    fn test_opcode_gtq_equal_true() {
        let mut vm = VM::new();
        vm.registers[0] = 6;
        vm.registers[1] = 6;
        vm.program = vec![13, 0, 1, 0]; // GTQ $0 $1
        vm.run_once();
        assert!(vm.equal_flag);
    }

    #[test]
    fn test_opcode_gtq_false() {
        let mut vm = VM::new();
        vm.registers[0] = 2;
        vm.registers[1] = 4;
        vm.program = vec![13, 0, 1, 0]; // GTQ $0 $1
        vm.run_once();
        assert!(!vm.equal_flag);
    }

    #[test]
    fn test_opcode_ltq_less_true() {
        let mut vm = VM::new();
        vm.registers[0] = 5;
        vm.registers[1] = 6;
        vm.program = vec![14, 0, 1, 0]; // LTQ $0 $1
        vm.run_once();
        assert!(vm.equal_flag);
    }

    #[test]
    fn test_opcode_ltq_equal_true() {
        let mut vm = VM::new();
        vm.registers[0] = 6;
        vm.registers[1] = 6;
        vm.program = vec![14, 0, 1, 0]; // LTQ $0 $1
        vm.run_once();
        assert!(vm.equal_flag);
    }

    #[test]
    fn test_opcode_ltq_false() {
        let mut vm = VM::new();
        vm.registers[0] = 4;
        vm.registers[1] = 2;
        vm.program = vec![14, 0, 1, 0]; // LTQ $0 $1
        vm.run_once();
        assert!(!vm.equal_flag);
    }

    #[test]
    fn test_opcode_jeq() {
        let mut vm = VM::new();
        vm.registers[2] = 4;
        vm.equal_flag = true;
        vm.program = vec![15, 2, 0, 0]; // JEQ $0
        vm.run_once();
        assert_eq!(vm.program_counter, 4);
    }

    #[test]
    fn test_opcode_jneq() {
        let mut vm = VM::new();
        vm.registers[2] = 4;
        vm.equal_flag = false;
        vm.program = vec![16, 2, 0, 0]; // JEQ $0
        vm.run_once();
        assert_eq!(vm.program_counter, 4);
    }
}
