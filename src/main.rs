use repl::REPL;

pub mod assembler;
pub mod instruction;
pub mod repl;
pub mod vm;

fn main() {
    let mut repl = REPL::new();
    repl.run();
}
