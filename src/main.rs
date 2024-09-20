pub mod assembler;
pub mod cli;
pub mod instruction;
pub mod repl;
pub mod vm;

fn main() {
    cli::run();
}
