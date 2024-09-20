use crate::{assembler::assembler::Assembler, repl::REPL, vm::VM};

use clap::{Arg, Command};
use std::{fs::File, io::Read, path::Path, process};

pub fn run() {
    let matches = Command::new("VMariachi")
        .version("1.0")
        .about("A 32-bit registered based Virtual Machine")
        .arg(Arg::new("file").short('f').long("file"))
        .get_matches();

    match matches.get_one::<String>("file") {
        Some(file) => {
            println!(">> reading file {file}");

            let program = read_file(file);
            let mut assembler = Assembler::new();
            let mut vm = VM::new();

            println!(">> assembling program");
            if let Some(bytes) = assembler.assemble(&program) {
                vm.add_program(bytes);

                println!(">> running program");
                vm.run();

                println!(">> completed!");
                process::exit(0);
            }
        }
        None => {
            let mut repl = REPL::new();
            repl.run();
        }
    }
}

fn read_file(file: &str) -> String {
    let mut f = File::open(Path::new(file.trim())).expect("Unable to open file");
    let mut content = String::new();
    f.read_to_string(&mut content).expect("Unable to read file");

    content
}
