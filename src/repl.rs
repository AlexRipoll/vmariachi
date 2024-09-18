use std::{
    fs::File,
    io::{self, Read, Write},
    num::ParseIntError,
    path::Path,
    process,
};

use crate::{assembler::Program, vm::VM};

#[derive(Debug, Default)]
pub struct REPL {
    vm: VM,
    command_buffer: Vec<String>,
}

impl REPL {
    pub fn new() -> Self {
        Self {
            vm: VM::new(),
            command_buffer: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            print!(">>> ");
            io::stdout().flush().expect("Unable to flush to stdout");

            // Wait for user input
            let stdin = io::stdin();
            let mut input = String::new();
            stdin
                .read_line(&mut input)
                .expect("Unable to read user input");

            let command = input.trim();
            self.command_buffer.push(command.to_string());

            match command {
                "!program" => {
                    self.vm.program.iter().for_each(|byte| println!("{}", byte));

                    println!("End of program");
                }
                "!registers" => {
                    println!("{:#?}", self.vm.registers);
                    println!("End of registers");
                }
                "!load_file" => {
                    print!("Enter the path of the file: ");
                    io::stdout().flush().expect("Unable to flush to stdout");

                    let mut tmp = String::new();
                    io::stdin()
                        .read_line(&mut tmp)
                        .expect("Unable to read user input");

                    let mut f = File::open(Path::new(tmp.trim())).expect("Unable to open file");
                    let mut content = String::new();
                    f.read_to_string(&mut content).expect("Unable to read file");

                    let (_, program) = match Program::parse(&content) {
                        Ok(n) => n,
                        Err(e) => {
                            eprintln!("Unable to parse input: {}", e);
                            continue;
                        }
                    };

                    let bytes = match program.to_bytes() {
                        Ok(b) => b,
                        Err(e) => {
                            eprintln!("{}", e);
                            continue;
                        }
                    };

                    self.vm.program.extend_from_slice(&bytes);
                }
                "!quit" => {
                    println!("My work is done, I quit");
                    process::exit(0);
                }
                "!history" => {
                    self.command_buffer.iter().for_each(|cmd| println!("{cmd}"));
                }
                "!clear" => {
                    self.vm.program.clear();
                }
                _ => {
                    let (_, program) = match Program::parse(command) {
                        Ok(n) => n,
                        Err(e) => {
                            eprintln!("{}", e);
                            continue;
                        }
                    };

                    let bytes = match program.to_bytes() {
                        Ok(b) => b,
                        Err(e) => {
                            eprintln!("{}", e);
                            continue;
                        }
                    };

                    self.vm.program.extend_from_slice(&bytes);

                    // hex instruction
                    //
                    // match self.parse_hex(&command) {
                    //     Ok(instruction) => self.vm.program.extend_from_slice(&instruction),
                    //     Err(_) => {
                    //         eprintln!(
                    //             "Error: Invalid hexadecimal instruction provided. The input must consist of 4 bytes in hexadecimal format, separated by spaces (e.g., '2A 00 02 FA'). Each byte should be a two-digit hexadecimal number."
                    //         )
                    //     }
                    // }

                    self.vm.run_once();
                }
            }
        }
    }

    fn parse_hex(&mut self, input: &str) -> Result<Vec<u8>, ParseIntError> {
        input
            .split(' ')
            .map(|hex_number| u8::from_str_radix(hex_number, 16))
            .collect()
    }
}
