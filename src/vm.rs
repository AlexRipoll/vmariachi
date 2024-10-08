use std::usize;

use crate::{assembler::assembler::PIE_HEADER_PREFIX, instruction::Opcode};

#[derive(Debug, Default)]
pub struct VM {
    pub registers: [i32; 32],
    pub program: Vec<u8>,
    program_counter: usize,
    heap: Vec<u8>,
    remainder: u32,
    equal_flag: bool,
}

impl VM {
    pub fn new() -> Self {
        Self {
            registers: [0; 32],
            program: Vec::new(),
            program_counter: 0,
            heap: Vec::new(),
            remainder: 0,
            equal_flag: false,
        }
    }

    pub fn run(&mut self) {
        if !self.has_valid_header() {
            eprintln!("Invalid header");
            return;
        }
        // skip remaining heder bytes
        self.program_counter = 64;

        while self.execute_instruction().is_some() {
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
                let register_idx = self.next_8_bits() as usize;
                let number = self.next_16_bits();
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
            Opcode::GTE => {
                let first_value = self.registers[self.next_8_bits() as usize];
                let second_value = self.registers[self.next_8_bits() as usize];
                self.equal_flag = first_value >= second_value;
                self.next_8_bits();
            }
            Opcode::LTE => {
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
            Opcode::ALOC => {
                let register = self.next_8_bits() as usize;
                let bytes = self.registers[register];
                self.heap.resize(self.heap.len() + bytes as usize, 0);
            }
            Opcode::INC => {
                let register = self.next_8_bits() as usize;
                self.registers[register] += 1;
            }
            Opcode::DEC => {
                let register = self.next_8_bits() as usize;
                self.registers[register] -= 1;
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

    pub fn add_program(&mut self, bytes: Vec<u8>) {
        self.program.extend_from_slice(&bytes);
    }

    fn has_valid_header(&self) -> bool {
        self.program[..4] == PIE_HEADER_PREFIX
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
            13 => Opcode::GTE,
            14 => Opcode::LTE,
            15 => Opcode::JEQ,
            16 => Opcode::JNEQ,
            17 => Opcode::ALOC,
            18 => Opcode::INC,
            19 => Opcode::DEC,
            _ => Opcode::IGL,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        assembler::assembler::{PIE_HEADER_LENGTH, PIE_HEADER_PREFIX},
        vm::VM,
    };

    fn prepend_header(mut program_body: Vec<u8>) -> Vec<u8> {
        let mut header = [0u8; PIE_HEADER_LENGTH];
        header[..4].copy_from_slice(&PIE_HEADER_PREFIX);
        let mut program = header.to_vec();
        program.append(&mut program_body);

        program
    }

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
        vm.program = prepend_header(vec![0, 0, 1, 244]); // LOAD $0 #500
        vm.program.extend_from_slice(&vec![0, 1, 0, 7]); // LOAD $1 #7
        vm.program.extend_from_slice(&vec![1, 0, 1, 2]); // ADD $0 $1 $2 (ADD  registers 0 and 1 and set result to register 2)
        vm.run();
        assert_eq!(vm.registers[2], 507);
    }

    #[test]
    fn test_opcode_sub() {
        let mut vm = VM::new();
        // [opcode, register, operand, operand]
        vm.program = prepend_header(vec![0, 0, 1, 244]); // LOAD $0 #500
        vm.program.extend_from_slice(&vec![0, 1, 0, 7]); // LOAD $1 #7
        vm.program.extend_from_slice(&vec![2, 0, 1, 2]); // SUB $0 $1 $2 (ADD  registers 0 and 1 and set result to register 2)
        vm.run();
        assert_eq!(vm.registers[2], 493);
    }

    #[test]
    fn test_opcode_mul() {
        let mut vm = VM::new();
        // [opcode, register, operand, operand]
        vm.program = prepend_header(vec![0, 0, 1, 244]); // LOAD $0 #500
        vm.program.extend_from_slice(&vec![0, 1, 0, 7]); // LOAD $1 #7
        vm.program.extend_from_slice(&vec![3, 0, 1, 2]); // MUL $0 $1 $2 (ADD  registers 0 and 1 and set result to register 2)
        vm.run();
        assert_eq!(vm.registers[2], 3500);
    }

    #[test]
    fn test_opcode_div_without_remainder() {
        let mut vm = VM::new();
        // [opcode, register, operand, operand]
        vm.program = prepend_header(vec![0, 0, 1, 244]); // LOAD $0 #500
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
        vm.program = prepend_header(vec![0, 0, 1, 244]); // LOAD $0 #500
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
    fn test_opcode_gte_greater_true() {
        let mut vm = VM::new();
        vm.registers[0] = 6;
        vm.registers[1] = 5;
        vm.program = vec![13, 0, 1, 0]; // GTE $0 $1
        vm.run_once();
        assert!(vm.equal_flag);
    }

    #[test]
    fn test_opcode_gte_equal_true() {
        let mut vm = VM::new();
        vm.registers[0] = 6;
        vm.registers[1] = 6;
        vm.program = vec![13, 0, 1, 0]; // GTE $0 $1
        vm.run_once();
        assert!(vm.equal_flag);
    }

    #[test]
    fn test_opcode_gte_false() {
        let mut vm = VM::new();
        vm.registers[0] = 2;
        vm.registers[1] = 4;
        vm.program = vec![13, 0, 1, 0]; // GTE $0 $1
        vm.run_once();
        assert!(!vm.equal_flag);
    }

    #[test]
    fn test_opcode_lte_less_true() {
        let mut vm = VM::new();
        vm.registers[0] = 5;
        vm.registers[1] = 6;
        vm.program = vec![14, 0, 1, 0]; // LTE $0 $1
        vm.run_once();
        assert!(vm.equal_flag);
    }

    #[test]
    fn test_opcode_lte_equal_true() {
        let mut vm = VM::new();
        vm.registers[0] = 6;
        vm.registers[1] = 6;
        vm.program = vec![14, 0, 1, 0]; // LTE $0 $1
        vm.run_once();
        assert!(vm.equal_flag);
    }

    #[test]
    fn test_opcode_lte_false() {
        let mut vm = VM::new();
        vm.registers[0] = 4;
        vm.registers[1] = 2;
        vm.program = vec![14, 0, 1, 0]; // LTE $0 $1
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

    #[test]
    fn test_opcode_aloc_on_empty_heap() {
        let mut vm = VM::new();
        vm.registers[0] = 1024;
        vm.program = vec![17, 0, 0, 0]; // ALOC $0
        vm.run_once();
        assert_eq!(vm.heap.len(), 1024);
    }

    #[test]
    fn test_opcode_aloc_extend_heap() {
        let mut vm = VM::new();
        vm.heap.extend_from_slice(&[0u8; 8]);
        vm.registers[0] = 1024;
        vm.program = vec![17, 0, 0, 0]; // ALOC $0
        vm.run_once();
        assert_eq!(vm.heap.len(), 1032);
    }

    #[test]
    fn test_opcode_inc() {
        let mut vm = VM::new();
        println!("=>> {}", vm.program_counter);
        vm.registers[0] = 1024;
        vm.program = vec![18, 0, 0, 0]; // INC $0
        vm.run_once();
        println!("{:?}", vm.registers);
        assert_eq!(vm.registers[0], 1025);
    }

    #[test]
    fn test_opcode_dec() {
        let mut vm = VM::new();
        vm.registers[0] = 1024;
        vm.program = vec![19, 0, 0, 0]; // DEC $0
        vm.run_once();
        assert_eq!(vm.registers[0], 1023);
    }

    #[test]
    fn test_add_program() {
        let mut vm = VM::new();
        let bytes = vec![19, 0, 0, 0]; // DEC $0
        vm.add_program(bytes.clone());
        assert_eq!(vm.program, bytes);
    }

    #[test]
    fn test_extend_program() {
        let mut vm = VM::new();
        vm.program = vec![18, 0, 0, 0]; // INC $0
        let bytes = vec![19, 0, 0, 0]; // DEC $0
        vm.add_program(bytes.clone());
        assert_eq!(vm.program, vec![18, 0, 0, 0, 19, 0, 0, 0]);
    }

    #[test]
    fn test_valid_header_true() {
        let mut vm = VM::new();
        let mut header = [0u8; 64];
        header[..4].copy_from_slice(&PIE_HEADER_PREFIX);
        let mut program = header.to_vec();
        program.append(&mut vec![18, 0, 0, 0, 19, 0, 0, 0]);
        vm.program = program;
        assert!(vm.has_valid_header());
    }

    #[test]
    fn test_valid_header_false() {
        let mut vm = VM::new();
        let header = [0u8; 64];
        let mut program = header.to_vec();
        program.append(&mut vec![18, 0, 0, 0, 19, 0, 0, 0]);
        vm.program = program;
        assert!(!vm.has_valid_header());
    }
}
