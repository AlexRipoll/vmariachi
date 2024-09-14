#[derive(Debug)]
pub struct VM {
    registers: [i32; 32],
}

impl VM {
    pub fn new() -> Self {
        Self { registers: [0; 32] }
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
}
