use super::parser::Program;

#[derive(Debug)]
pub struct Assembler {
    phase: AssemblerPhase,
    symbols: SymbolTable,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            phase: AssemblerPhase::First,
            symbols: SymbolTable::new(),
        }
    }

    pub fn assemble(&mut self, raw: &str) -> Option<Vec<u8>> {
        Program::parse(raw).map_or_else(
            |e| {
                println!("There was an error assembling the code: {:?}", e);
                None
            },
            |(_remainder, program)| {
                self.process_first_phase(&program);
                self.process_second_phase(&program).ok()
            },
        )
    }

    fn process_first_phase(&mut self, p: &Program) {
        self.extract_labels(p);
        self.phase = AssemblerPhase::Second;
    }

    fn process_second_phase(&mut self, p: &Program) -> Result<Vec<u8>, String> {
        let mut program = Vec::new();
        for instruction in &p.instructions {
            let mut bytes = instruction.to_bytes()?;
            program.append(&mut bytes);
        }

        Ok(program)
    }

    fn extract_labels(&mut self, p: &Program) {
        let mut offset = 0;
        for instruction in &p.instructions {
            if instruction.is_label() {
                if let Some(name) = instruction.label_name() {
                    let symbol = Symbol::new(name, SymbolType::Label, offset);
                    self.symbols.add_symbol(symbol);
                }
            }
            offset += 4;
        }
    }
}

#[derive(Debug)]
pub struct Symbol {
    name: String,
    offset: u32,
    symbol_type: SymbolType,
}

impl Symbol {
    fn new(name: String, symbol_type: SymbolType, offset: u32) -> Symbol {
        Symbol {
            name,
            symbol_type,
            offset,
        }
    }
}

#[derive(Debug)]
pub struct SymbolTable {
    symbols: Vec<Symbol>,
}

impl SymbolTable {
    fn new() -> SymbolTable {
        SymbolTable {
            symbols: Vec::new(),
        }
    }

    fn add_symbol(&mut self, s: Symbol) {
        self.symbols.push(s);
    }

    fn symbol_offset(&self, s: &str) -> Option<u32> {
        self.symbols
            .iter()
            .find(|&symbol| symbol.name == s)
            .map(|symbol| symbol.offset)
    }
}

#[derive(Debug)]
enum AssemblerPhase {
    First,
    Second,
}

#[derive(Debug)]
enum SymbolType {
    Label,
}

#[cfg(test)]
mod test {
    use crate::assembler::assembler::{Assembler, SymbolTable};

    use super::{Symbol, SymbolType};

    #[test]
    fn test_symbol_table_add() {
        let mut symbol_table = SymbolTable::new();
        let new_symbol = Symbol::new("test".to_string(), SymbolType::Label, 12);
        symbol_table.add_symbol(new_symbol);
        assert_eq!(symbol_table.symbols.len(), 1);
    }

    #[test]
    fn test_symbol_table_offset() {
        let mut symbol_table = SymbolTable::new();
        let new_symbol = Symbol::new("test".to_string(), SymbolType::Label, 12);
        symbol_table.add_symbol(new_symbol);
        let offset = symbol_table.symbol_offset("test").unwrap();
        assert_eq!(offset, 12);
    }

    #[test]
    fn test_assembler() {
        let mut assembler = Assembler::new();
        let raw_instructions =
            "load $0 #100\nload $1 #1\nload $2 #0\ntest: inc $0\nneq $0 $2\njeq @test\nhlt";
        let program_bytes = assembler.assemble(raw_instructions).unwrap();
        assert_eq!(program_bytes.len(), 28);
    }
}
