use crate::opcode::{Opcode, VmError};

// ─── VM ──────────────────────────────────────────────────────────────────────
//
// Stack-based virtual machine.
//
// Design hints:
//   - `stack`:   Vec<i64>        — operand stack (bottom → top)
//   - `regs`:    Vec<i64>        — general-purpose registers (initialised to 0)
//   - `output`:  Vec<i64>        — Print output log
//   - `pc`:      usize           — program counter (index into program)
//
// execute() runs a loop:
//   1. Fetch program[pc] (error if pc is out of bounds; usually means no Halt).
//   2. Increment pc.
//   3. Dispatch on the opcode.
//   For binary arithmetic/comparison: pop rhs first, then lhs.
//   For jumps: set pc directly (don't increment before the jump).

pub struct Vm {
    // TODO: stack, regs, output, pc
}

impl Vm {
    /// Create a VM with `num_registers` general-purpose registers (all zero).
    pub fn new(num_registers: usize) -> Self {
        todo!()
    }

    /// Execute `program` from instruction 0 until Halt or end-of-program.
    pub fn execute(&mut self, program: &[Opcode]) -> Result<(), VmError> {
        todo!()
    }

    /// Current stack contents (bottom → top, read-only).
    pub fn stack(&self) -> &[i64] {
        todo!()
    }

    /// Read register `idx`.  Returns None if the index is out of range.
    pub fn register(&self, idx: usize) -> Option<i64> {
        todo!()
    }

    /// Values recorded by Print instructions, in execution order.
    pub fn output(&self) -> &[i64] {
        todo!()
    }

    /// Reset to initial state: clear stack, zero all registers, clear output.
    pub fn reset(&mut self) {
        todo!()
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use Opcode::*;

    // ── Stack ops ─────────────────────────────────────────────────────────────

    #[test] fn test_push_leaves_value() {
        let mut vm = Vm::new(0);
        vm.execute(&[Push(7), Halt]).unwrap();
        assert_eq!(vm.stack(), &[7]);
    }

    #[test] fn test_pop_removes_top() {
        let mut vm = Vm::new(0);
        vm.execute(&[Push(1), Push(2), Pop, Halt]).unwrap();
        assert_eq!(vm.stack(), &[1]);
    }

    #[test] fn test_pop_underflow() {
        let mut vm = Vm::new(0);
        assert_eq!(vm.execute(&[Pop]), Err(VmError::StackUnderflow));
    }

    #[test] fn test_dup_copies_top() {
        let mut vm = Vm::new(0);
        vm.execute(&[Push(5), Dup, Halt]).unwrap();
        assert_eq!(vm.stack(), &[5, 5]);
    }

    #[test] fn test_swap_exchanges_top_two() {
        let mut vm = Vm::new(0);
        vm.execute(&[Push(1), Push(2), Swap, Halt]).unwrap();
        assert_eq!(vm.stack(), &[2, 1]);
    }

    // ── Arithmetic ────────────────────────────────────────────────────────────

    #[test] fn test_add() {
        let mut vm = Vm::new(0);
        vm.execute(&[Push(3), Push(4), Add, Halt]).unwrap();
        assert_eq!(vm.stack(), &[7]);
    }

    #[test] fn test_sub() {
        let mut vm = Vm::new(0);
        vm.execute(&[Push(10), Push(3), Sub, Halt]).unwrap();
        assert_eq!(vm.stack(), &[7]);
    }

    #[test] fn test_mul() {
        let mut vm = Vm::new(0);
        vm.execute(&[Push(6), Push(7), Mul, Halt]).unwrap();
        assert_eq!(vm.stack(), &[42]);
    }

    #[test] fn test_div() {
        let mut vm = Vm::new(0);
        vm.execute(&[Push(20), Push(4), Div, Halt]).unwrap();
        assert_eq!(vm.stack(), &[5]);
    }

    #[test] fn test_div_by_zero() {
        let mut vm = Vm::new(0);
        assert_eq!(vm.execute(&[Push(5), Push(0), Div]), Err(VmError::DivisionByZero));
    }

    #[test] fn test_mod() {
        let mut vm = Vm::new(0);
        vm.execute(&[Push(10), Push(3), Mod, Halt]).unwrap();
        assert_eq!(vm.stack(), &[1]);
    }

    #[test] fn test_neg() {
        let mut vm = Vm::new(0);
        vm.execute(&[Push(5), Neg, Halt]).unwrap();
        assert_eq!(vm.stack(), &[-5]);
    }

    // ── Comparison ────────────────────────────────────────────────────────────

    #[test] fn test_eq_true()  { let mut vm = Vm::new(0); vm.execute(&[Push(3), Push(3), Eq, Halt]).unwrap(); assert_eq!(vm.stack(), &[1]); }
    #[test] fn test_eq_false() { let mut vm = Vm::new(0); vm.execute(&[Push(3), Push(4), Eq, Halt]).unwrap(); assert_eq!(vm.stack(), &[0]); }
    #[test] fn test_ne_true()  { let mut vm = Vm::new(0); vm.execute(&[Push(1), Push(2), Ne, Halt]).unwrap(); assert_eq!(vm.stack(), &[1]); }
    #[test] fn test_lt_true()  { let mut vm = Vm::new(0); vm.execute(&[Push(2), Push(5), Lt, Halt]).unwrap(); assert_eq!(vm.stack(), &[1]); }
    #[test] fn test_le_equal() { let mut vm = Vm::new(0); vm.execute(&[Push(3), Push(3), Le, Halt]).unwrap(); assert_eq!(vm.stack(), &[1]); }
    #[test] fn test_gt_true()  { let mut vm = Vm::new(0); vm.execute(&[Push(5), Push(2), Gt, Halt]).unwrap(); assert_eq!(vm.stack(), &[1]); }
    #[test] fn test_ge_equal() { let mut vm = Vm::new(0); vm.execute(&[Push(4), Push(4), Ge, Halt]).unwrap(); assert_eq!(vm.stack(), &[1]); }

    // ── Control flow ──────────────────────────────────────────────────────────

    #[test]
    fn test_jump_skips_instructions() {
        let mut vm = Vm::new(0);
        // idx 0=Jump(2), 1=Push(99), 2=Push(1), 3=Halt
        vm.execute(&[Jump(2), Push(99), Push(1), Halt]).unwrap();
        assert_eq!(vm.stack(), &[1]);
    }

    #[test]
    fn test_jump_if_taken_when_nonzero() {
        let mut vm = Vm::new(0);
        // 0=Push(1), 1=JumpIf(3), 2=Push(99), 3=Push(42), 4=Halt
        vm.execute(&[Push(1), JumpIf(3), Push(99), Push(42), Halt]).unwrap();
        assert!(!vm.stack().contains(&99));
        assert!(vm.stack().contains(&42));
    }

    #[test]
    fn test_jump_if_not_taken_when_zero() {
        let mut vm = Vm::new(0);
        // 0=Push(0), 1=JumpIfNot(3), 2=Push(99), 3=Push(7), 4=Halt
        vm.execute(&[Push(0), JumpIfNot(3), Push(99), Push(7), Halt]).unwrap();
        assert!(!vm.stack().contains(&99));
        assert!(vm.stack().contains(&7));
    }

    // ── Registers ─────────────────────────────────────────────────────────────

    #[test]
    fn test_store_and_load() {
        let mut vm = Vm::new(4);
        vm.execute(&[Push(55), Store(2), Load(2), Halt]).unwrap();
        assert_eq!(vm.register(2), Some(55));
        assert_eq!(vm.stack(), &[55]);
    }

    #[test]
    fn test_invalid_register() {
        let mut vm = Vm::new(2);
        let err = vm.execute(&[Push(1), Store(5)]);
        assert!(matches!(err, Err(VmError::InvalidRegister { index: 5 })));
    }

    // ── Print ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_print_records_output() {
        let mut vm = Vm::new(0);
        vm.execute(&[Push(10), Print, Push(20), Print, Halt]).unwrap();
        assert_eq!(vm.output(), &[10, 20]);
        assert!(vm.stack().is_empty(), "Print must consume the value");
    }

    // ── Integration: sum 1..=5 via loop ───────────────────────────────────────
    //
    //  reg[0] = accumulator (0 → 15)
    //  reg[1] = counter     (5 → 0)
    //
    //  0:  Push(0),  Store(0)
    //  2:  Push(5),  Store(1)
    //  4:  Load(1),  JumpIfNot(15)
    //  6:  Load(0),  Load(1), Add, Store(0)
    //  10: Load(1),  Push(1), Sub, Store(1)
    //  14: Jump(4)
    //  15: Load(0),  Print, Halt
    #[test]
    fn test_sum_1_to_5_loop() {
        let mut vm = Vm::new(2);
        let prog = vec![
            Push(0), Store(0),
            Push(5), Store(1),
            Load(1), JumpIfNot(15),
            Load(0), Load(1), Add, Store(0),
            Load(1), Push(1), Sub, Store(1),
            Jump(4),
            Load(0), Print,
            Halt,
        ];
        vm.execute(&prog).unwrap();
        assert_eq!(vm.output(), &[15]);
    }

    // ── reset ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_reset_clears_all_state() {
        let mut vm = Vm::new(2);
        vm.execute(&[Push(99), Store(1), Push(7), Print, Halt]).unwrap();
        vm.reset();
        assert!(vm.stack().is_empty());
        assert_eq!(vm.register(0), Some(0));
        assert_eq!(vm.register(1), Some(0));
        assert!(vm.output().is_empty());
    }
}
