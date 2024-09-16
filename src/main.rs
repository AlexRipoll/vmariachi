use repl::REPL;

pub mod instruction;
pub mod repl;
pub mod vm;

fn main() {
    let mut repl = REPL::new();
    repl.run();
}
