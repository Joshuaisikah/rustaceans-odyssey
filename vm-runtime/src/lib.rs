// ─── Instruction set ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    // Stack
    Push(i64),
    Pop,
    Dup,   // duplicate top
    Swap,  // swap top two
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg, // negate top
    // Comparison  (push 1 for true, 0 for false)
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    // Control flow
    Jump(usize),       // unconditional jump to instruction index
    JumpIf(usize),     // pop; jump if non-zero
    JumpIfNot(usize),  // pop; jump if zero
    // Registers
    Load(usize),  // push register[n] onto stack
    Store(usize), // pop and store into register[n]
    // Output
    Print, // pop and record in output log
    // Halt
    Halt,
}

// ─── Errors ──────────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq)]
pub enum VmError {
    StackUnderflow,
    DivisionByZero,
    InvalidJump { target: usize, len: usize },
    InvalidRegister { index: usize },
}

// ─── VM ──────────────────────────────────────────────────────────────────────

pub struct Vm {
    // TODO: add your fields (stack, registers, program counter, output log, …)
}

impl Vm {
    /// Create a VM with `num_registers` general-purpose registers.
    pub fn new(num_registers: usize) -> Self {
        todo!()
    }

    /// Execute `program` from instruction 0 until `Halt` or end of program.
    pub fn execute(&mut self, program: &[Opcode]) -> Result<(), VmError> {
        todo!()
    }

    /// View the current stack (bottom → top).
    pub fn stack(&self) -> &[i64] {
        todo!()
    }

    /// Read a register value.
    pub fn register(&self, idx: usize) -> Option<i64> {
        todo!()
    }

    /// Values recorded by `Print` instructions in execution order.
    pub fn output(&self) -> &[i64] {
        todo!()
    }

    /// Reset the VM to its initial state (clears stack, registers, output, pc).
    pub fn reset(&mut self) {
        todo!()
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use Opcode::*;

    // Push places a value on the stack.
    #[test]
    fn test_push_leaves_value_on_stack() {
        let mut vm = Vm::new(0);
        vm.execute(&[Push(7), Halt]).unwrap();
        assert_eq!(vm.stack(), &[7]);
    }

    // Pop removes the top of the stack.
    #[test]
    fn test_pop_removes_top() {
        let mut vm = Vm::new(0);
        vm.execute(&[Push(1), Push(2), Pop, Halt]).unwrap();
        assert_eq!(vm.stack(), &[1]);
    }

    // Pop on an empty stack returns StackUnderflow.
    #[test]
    fn test_pop_empty_stack_returns_underflow() {
        let mut vm = Vm::new(0);
        let err = vm.execute(&[Pop]);
        assert_eq!(err, Err(VmError::StackUnderflow));
    }

    // Dup copies the top element.
    #[test]
    fn test_dup_copies_top() {
        let mut vm = Vm::new(0);
        vm.execute(&[Push(5), Dup, Halt]).unwrap();
        assert_eq!(vm.stack(), &[5, 5]);
    }

    // Swap exchanges the two top elements.
    #[test]
    fn test_swap_exchanges_top_two() {
        let mut vm = Vm::new(0);
        vm.execute(&[Push(1), Push(2), Swap, Halt]).unwrap();
        assert_eq!(vm.stack(), &[2, 1]);
    }

    // Basic arithmetic: Add
    #[test]
    fn test_add() {
        let mut vm = Vm::new(0);
        vm.execute(&[Push(3), Push(4), Add, Halt]).unwrap();
        assert_eq!(vm.stack(), &[7]);
    }

    // Sub
    #[test]
    fn test_sub() {
        let mut vm = Vm::new(0);
        vm.execute(&[Push(10), Push(3), Sub, Halt]).unwrap();
        assert_eq!(vm.stack(), &[7]);
    }

    // Mul
    #[test]
    fn test_mul() {
        let mut vm = Vm::new(0);
        vm.execute(&[Push(6), Push(7), Mul, Halt]).unwrap();
        assert_eq!(vm.stack(), &[42]);
    }

    // Div
    #[test]
    fn test_div() {
        let mut vm = Vm::new(0);
        vm.execute(&[Push(20), Push(4), Div, Halt]).unwrap();
        assert_eq!(vm.stack(), &[5]);
    }

    // Div by zero returns DivisionByZero.
    #[test]
    fn test_div_by_zero_returns_error() {
        let mut vm = Vm::new(0);
        let err = vm.execute(&[Push(5), Push(0), Div]);
        assert_eq!(err, Err(VmError::DivisionByZero));
    }

    // Mod
    #[test]
    fn test_mod() {
        let mut vm = Vm::new(0);
        vm.execute(&[Push(10), Push(3), Mod, Halt]).unwrap();
        assert_eq!(vm.stack(), &[1]);
    }

    // Neg negates the top value.
    #[test]
    fn test_neg() {
        let mut vm = Vm::new(0);
        vm.execute(&[Push(5), Neg, Halt]).unwrap();
        assert_eq!(vm.stack(), &[-5]);
    }

    // Eq: equal values push 1.
    #[test]
    fn test_eq_true() {
        let mut vm = Vm::new(0);
        vm.execute(&[Push(3), Push(3), Eq, Halt]).unwrap();
        assert_eq!(vm.stack(), &[1]);
    }

    // Eq: unequal values push 0.
    #[test]
    fn test_eq_false() {
        let mut vm = Vm::new(0);
        vm.execute(&[Push(3), Push(4), Eq, Halt]).unwrap();
        assert_eq!(vm.stack(), &[0]);
    }

    // Lt: 2 < 5 → 1.
    #[test]
    fn test_lt_true() {
        let mut vm = Vm::new(0);
        vm.execute(&[Push(2), Push(5), Lt, Halt]).unwrap();
        assert_eq!(vm.stack(), &[1]);
    }

    // Gt: 5 > 2 → 1.
    #[test]
    fn test_gt_true() {
        let mut vm = Vm::new(0);
        vm.execute(&[Push(5), Push(2), Gt, Halt]).unwrap();
        assert_eq!(vm.stack(), &[1]);
    }

    // Unconditional jump.
    #[test]
    fn test_jump_skips_instructions() {
        let mut vm = Vm::new(0);
        // index: 0=Jump(2), 1=Push(99), 2=Push(1), 3=Halt
        vm.execute(&[Jump(2), Push(99), Push(1), Halt]).unwrap();
        // Push(99) at index 1 was skipped
        assert_eq!(vm.stack(), &[1]);
    }

    // JumpIf: condition = 1 → take the jump.
    #[test]
    fn test_jump_if_taken_when_nonzero() {
        let mut vm = Vm::new(0);
        // 0=Push(1), 1=JumpIf(3), 2=Push(99), 3=Push(42), 4=Halt
        vm.execute(&[Push(1), JumpIf(3), Push(99), Push(42), Halt]).unwrap();
        assert!(!vm.stack().contains(&99), "skipped instruction must not execute");
        assert!(vm.stack().contains(&42));
    }

    // JumpIfNot: condition = 0 → take the jump.
    #[test]
    fn test_jump_if_not_taken_when_zero() {
        let mut vm = Vm::new(0);
        // 0=Push(0), 1=JumpIfNot(3), 2=Push(99), 3=Push(7), 4=Halt
        vm.execute(&[Push(0), JumpIfNot(3), Push(99), Push(7), Halt]).unwrap();
        assert!(!vm.stack().contains(&99));
        assert!(vm.stack().contains(&7));
    }

    // Store writes to a register; Load reads it back.
    #[test]
    fn test_store_and_load_register() {
        let mut vm = Vm::new(4);
        vm.execute(&[Push(55), Store(2), Load(2), Halt]).unwrap();
        assert_eq!(vm.register(2), Some(55));
        assert_eq!(vm.stack(), &[55]);
    }

    // Accessing an out-of-bounds register returns InvalidRegister.
    #[test]
    fn test_invalid_register_index() {
        let mut vm = Vm::new(2);
        let err = vm.execute(&[Push(1), Store(5)]);
        assert!(matches!(err, Err(VmError::InvalidRegister { index: 5 })));
    }

    // Print records the value in the output log without leaving it on the stack.
    #[test]
    fn test_print_records_output() {
        let mut vm = Vm::new(0);
        vm.execute(&[Push(10), Print, Push(20), Print, Halt]).unwrap();
        assert_eq!(vm.output(), &[10, 20]);
        assert!(vm.stack().is_empty(), "Print must consume the value");
    }

    // Program: compute 1+2+3+4+5 = 15 using a loop stored in registers.
    //
    //   reg[0] = accumulator (starts 0)
    //   reg[1] = counter     (starts 5, counts down)
    //
    //   loop:
    //     Load(1)          ; push counter
    //     JumpIfNot(end)   ; if counter == 0 goto end
    //     Load(0)          ; push acc
    //     Load(1)          ; push counter
    //     Add              ; acc + counter
    //     Store(0)         ; store new acc
    //     Load(1)          ; push counter
    //     Push(1)          ; push 1
    //     Sub              ; counter - 1
    //     Store(1)         ; store new counter
    //     Jump(loop_start) ; repeat
    //   end:
    //     Load(0)          ; push result
    //     Print
    //     Halt
    #[test]
    fn test_sum_1_to_5_loop_program() {
        let mut vm = Vm::new(2);

        // Initialise registers
        vm.execute(&[Push(0), Store(0), Push(5), Store(1), Halt]).unwrap();
        vm.reset();

        // Full program (indices 0-based)
        // We encode the loop manually:
        // 0: Push(0)  → acc = 0
        // 1: Store(0)
        // 2: Push(5)  → counter = 5
        // 3: Store(1)
        // --- loop start = 4 ---
        // 4: Load(1)
        // 5: JumpIfNot(16)   (exit when counter == 0)
        // 6: Load(0)
        // 7: Load(1)
        // 8: Add
        // 9: Store(0)
        // 10: Load(1)
        // 11: Push(1)
        // 12: Sub
        // 13: Store(1)
        // 14: Jump(4)
        // --- exit = 15 ---
        // 15: Load(0)
        // 16: Print
        // 17: Halt
        let program = vec![
            Push(0), Store(0),
            Push(5), Store(1),
            // loop
            Load(1), JumpIfNot(15),
            Load(0), Load(1), Add, Store(0),
            Load(1), Push(1), Sub, Store(1),
            Jump(4),
            // exit
            Load(0), Print,
            Halt,
        ];
        vm.execute(&program).unwrap();
        assert_eq!(vm.output(), &[15]);
    }

    // reset() clears stack, registers, and output.
    #[test]
    fn test_reset_clears_state() {
        let mut vm = Vm::new(2);
        vm.execute(&[Push(99), Store(1), Push(7), Print, Halt]).unwrap();
        vm.reset();
        assert!(vm.stack().is_empty());
        assert_eq!(vm.register(0), Some(0));
        assert_eq!(vm.register(1), Some(0));
        assert!(vm.output().is_empty());
    }
}
