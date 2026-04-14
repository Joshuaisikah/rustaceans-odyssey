// ─── vm-runtime: integration tests ───────────────────────────────────────────
//
// Black-box tests through the public API.
// Exercises Opcode, VmError, and Vm as an end-to-end bytecode system.

use vm_runtime::{Opcode, Vm, VmError};
use Opcode::*;

// ── Basic execution ───────────────────────────────────────────────────────────

#[test]
fn test_halt_stops_execution() {
    let mut vm = Vm::new(0);
    vm.execute(&[Push(99), Halt]).unwrap();
    assert_eq!(vm.stack(), &[99]);
}

#[test]
fn test_empty_program_terminates_cleanly() {
    let mut vm = Vm::new(0);
    // No instructions → implicit end-of-program; must not panic.
    let _ = vm.execute(&[]);
}

// ── Stack operations ──────────────────────────────────────────────────────────

#[test]
fn test_push_pop_sequence() {
    let mut vm = Vm::new(0);
    vm.execute(&[Push(1), Push(2), Push(3), Pop, Halt]).unwrap();
    assert_eq!(vm.stack(), &[1, 2]);
}

#[test]
fn test_dup_duplicates_top() {
    let mut vm = Vm::new(0);
    vm.execute(&[Push(7), Dup, Halt]).unwrap();
    assert_eq!(vm.stack(), &[7, 7]);
}

#[test]
fn test_swap_exchanges_top_two() {
    let mut vm = Vm::new(0);
    vm.execute(&[Push(1), Push(2), Swap, Halt]).unwrap();
    assert_eq!(vm.stack(), &[2, 1]);
}

// ── Arithmetic ────────────────────────────────────────────────────────────────

#[test]
fn test_arithmetic_chain() {
    // (3 + 4) * 2 - 1 = 13
    let mut vm = Vm::new(0);
    vm.execute(&[Push(3), Push(4), Add, Push(2), Mul, Push(1), Sub, Halt]).unwrap();
    assert_eq!(vm.stack(), &[13]);
}

#[test]
fn test_mod_operation() {
    let mut vm = Vm::new(0);
    vm.execute(&[Push(10), Push(3), Mod, Halt]).unwrap();
    assert_eq!(vm.stack(), &[1]);
}

#[test]
fn test_neg_negates_top() {
    let mut vm = Vm::new(0);
    vm.execute(&[Push(5), Neg, Halt]).unwrap();
    assert_eq!(vm.stack(), &[-5]);
}

#[test]
fn test_division_by_zero_error() {
    let mut vm = Vm::new(0);
    assert_eq!(vm.execute(&[Push(5), Push(0), Div]), Err(VmError::DivisionByZero));
}

#[test]
fn test_pop_underflow_error() {
    let mut vm = Vm::new(0);
    assert_eq!(vm.execute(&[Pop]), Err(VmError::StackUnderflow));
}

// ── Comparison ────────────────────────────────────────────────────────────────

#[test]
fn test_comparison_ops() {
    let cases: &[(&[Opcode], i64)] = &[
        (&[Push(3), Push(3), Eq, Halt], 1),
        (&[Push(3), Push(4), Eq, Halt], 0),
        (&[Push(1), Push(2), Ne, Halt], 1),
        (&[Push(2), Push(5), Lt, Halt], 1),
        (&[Push(3), Push(3), Le, Halt], 1),
        (&[Push(5), Push(2), Gt, Halt], 1),
        (&[Push(4), Push(4), Ge, Halt], 1),
    ];
    for (prog, expected) in cases {
        let mut vm = Vm::new(0);
        vm.execute(prog).unwrap();
        assert_eq!(vm.stack(), &[*expected], "program: {prog:?}");
    }
}

// ── Registers ────────────────────────────────────────────────────────────────

#[test]
fn test_store_and_load_registers() {
    let mut vm = Vm::new(4);
    vm.execute(&[Push(42), Store(2), Load(2), Halt]).unwrap();
    assert_eq!(vm.register(2), Some(42));
    assert_eq!(vm.stack(), &[42]);
}

#[test]
fn test_registers_initialised_to_zero() {
    let vm = Vm::new(4);
    for i in 0..4 {
        assert_eq!(vm.register(i), Some(0));
    }
    assert_eq!(vm.register(4), None); // out of range
}

#[test]
fn test_invalid_register_error() {
    let mut vm = Vm::new(2);
    assert!(matches!(
        vm.execute(&[Push(1), Store(5)]),
        Err(VmError::InvalidRegister { index: 5 })
    ));
}

// ── Control flow ──────────────────────────────────────────────────────────────

#[test]
fn test_unconditional_jump_skips_instructions() {
    let mut vm = Vm::new(0);
    // idx 0=Jump(2), 1=Push(99), 2=Push(1), 3=Halt
    vm.execute(&[Jump(2), Push(99), Push(1), Halt]).unwrap();
    assert!(!vm.stack().contains(&99));
}

#[test]
fn test_jump_if_taken_on_nonzero() {
    let mut vm = Vm::new(0);
    // 0=Push(1), 1=JumpIf(3), 2=Push(99 — skipped), 3=Push(42), 4=Halt
    vm.execute(&[Push(1), JumpIf(3), Push(99), Push(42), Halt]).unwrap();
    assert!(!vm.stack().contains(&99));
    assert!(vm.stack().contains(&42));
}

#[test]
fn test_jump_if_not_taken_on_zero() {
    let mut vm = Vm::new(0);
    vm.execute(&[Push(0), JumpIfNot(3), Push(99), Push(7), Halt]).unwrap();
    assert!(!vm.stack().contains(&99));
    assert!(vm.stack().contains(&7));
}

// ── Print ─────────────────────────────────────────────────────────────────────

#[test]
fn test_print_records_output_and_pops_stack() {
    let mut vm = Vm::new(0);
    vm.execute(&[Push(10), Print, Push(20), Print, Halt]).unwrap();
    assert_eq!(vm.output(), &[10, 20]);
    assert!(vm.stack().is_empty(), "Print must consume the value");
}

// ── Integration programs ──────────────────────────────────────────────────────

// Sum 1..=5 using a register-based loop (expected: 15).
//
// reg[0] = accumulator (starts 0)
// reg[1] = counter     (starts 5, decrements to 0)
//
// 0:  Push(0), Store(0)
// 2:  Push(5), Store(1)
// 4:  Load(1), JumpIfNot(15)     ← exit when counter == 0
// 6:  Load(0), Load(1), Add, Store(0)
// 10: Load(1), Push(1), Sub, Store(1)
// 14: Jump(4)
// 15: Load(0), Print, Halt
#[test]
fn test_sum_loop_1_to_5() {
    let mut vm = Vm::new(2);
    let prog = [
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

// Countdown 3..=1 — print each value as the counter decrements.
#[test]
fn test_countdown_loop() {
    let mut vm = Vm::new(1);
    // reg[0] = counter (3 → 0)
    // 0:  Push(3), Store(0)
    // 2:  Load(0), JumpIfNot(9)
    // 4:  Load(0), Print
    // 6:  Load(0), Push(1), Sub, Store(0)
    // 10: Jump(2)
    // 9:  Halt     ← note: index 9 is Halt
    let prog = [
        Push(3), Store(0),        // 0, 1
        Load(0), JumpIfNot(9),    // 2, 3
        Load(0), Print,           // 4, 5
        Load(0), Push(1), Sub, Store(0), // 6, 7, 8, 9 — wait, this is 4 ops at idx 6-9
        Jump(2),                  // idx 10
        Halt,                     // idx 11 — target was wrong
    ];
    // Recalculate: JumpIfNot target should be idx 11 (Halt).
    let prog = [
        Push(3), Store(0),               // 0, 1
        Load(0), JumpIfNot(11),          // 2, 3
        Load(0), Print,                  // 4, 5
        Load(0), Push(1), Sub, Store(0), // 6, 7, 8, 9
        Jump(2),                         // 10
        Halt,                            // 11
    ];
    vm.execute(&prog).unwrap();
    assert_eq!(vm.output(), &[3, 2, 1]);
}

// Fibonacci(6) = 8.
// reg[0] = a = 0, reg[1] = b = 1, reg[2] = n = 6
// Each iteration: tmp = a + b; a = b; b = tmp; n -= 1
//
// Offsets (each instruction is 1 slot):
// 0:  Push(0), Store(0)
// 2:  Push(1), Store(1)
// 4:  Push(6), Store(2)
// 6:  Load(2), JumpIfNot(exit)
// 8:  Load(0), Load(1), Add      ← tmp on stack
// 11: Load(1), Store(0)          ← a = b
// 13: Store(1)                   ← b = tmp (still on stack from idx 8-10)
// 14: Load(2), Push(1), Sub, Store(2)
// 18: Jump(6)
// exit (19): Load(1), Print, Halt
#[test]
fn test_fibonacci_6() {
    let mut vm = Vm::new(3);
    let prog = [
        Push(0), Store(0),               // 0, 1
        Push(1), Store(1),               // 2, 3
        Push(6), Store(2),               // 4, 5
        Load(2), JumpIfNot(19),          // 6, 7
        Load(0), Load(1), Add,           // 8, 9, 10   — stack: [tmp]
        Load(1), Store(0),               // 11, 12     — a = b
        Store(1),                        // 13         — b = tmp
        Load(2), Push(1), Sub, Store(2), // 14, 15, 16, 17
        Jump(6),                         // 18
        Load(1), Print,                  // 19, 20
        Halt,                            // 21
    ];
    vm.execute(&prog).unwrap();
    assert_eq!(vm.output(), &[8]); // fib(6) = 0,1,1,2,3,5,8
}

// ── Reset ─────────────────────────────────────────────────────────────────────

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

#[test]
fn test_vm_reusable_after_reset() {
    let mut vm = Vm::new(0);
    vm.execute(&[Push(1), Halt]).unwrap();
    vm.reset();
    vm.execute(&[Push(2), Halt]).unwrap();
    assert_eq!(vm.stack(), &[2]);
}

// ── Error display ─────────────────────────────────────────────────────────────

#[test]
fn test_vm_error_display_is_non_empty() {
    let errs = [
        VmError::StackUnderflow,
        VmError::DivisionByZero,
        VmError::InvalidJump     { target: 5, len: 3 },
        VmError::InvalidRegister { index: 99 },
    ];
    for e in errs {
        assert!(!e.to_string().is_empty(), "VmError display must not be empty for {e:?}");
    }
}
