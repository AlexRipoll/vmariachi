use std::{
    io::{self, Write},
    process,
};

use crate::vm::VM;

#[derive(Debug)]
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
                    self.vm
                        .program()
                        .iter()
                        .for_each(|byte| println!("{}", byte));

                    println!("End of program");
                }
                "!registers" => {
                    println!("{:#?}", self.vm.registers());
                    println!("End of registers");
                }
                "!quit" => {
                    println!("My work is done, I quit");
                    process::exit(0);
                }
                "!history" => {
                    self.command_buffer.iter().for_each(|cmd| println!("{cmd}"));
                }
                _ => {
                    println!("invalid command")
                }
            }
        }
    }
}
