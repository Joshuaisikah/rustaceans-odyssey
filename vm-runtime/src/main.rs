// ─── vm-runtime — integration demo ───────────────────────────────────────────
//
// Run with:  cargo run -p vm-runtime

use vm_runtime::{Opcode::*, Vm};

fn main() {
    println!("=== vm-runtime integration demo ===\n");

    demo_basic_arithmetic();
    demo_register_ops();
    demo_control_flow();
    demo_sum_loop();
    demo_fibonacci();

    println!("\nAll demos completed.");
}

// ── Demo 1: basic arithmetic ──────────────────────────────────────────────────
fn demo_basic_arithmetic() {
    println!("[ Demo 1 ] arithmetic");

    let mut vm = Vm::new(0);
    // (3 + 4) * 2 = 14
    vm.execute(&[Push(3), Push(4), Add, Push(2), Mul, Print, Halt]).unwrap();
    assert_eq!(vm.output(), &[14]);
    println!("  (3 + 4) * 2 = {}  ✓", vm.output()[0]);
}

// ── Demo 2: register store/load ───────────────────────────────────────────────
fn demo_register_ops() {
    println!("[ Demo 2 ] register store / load");

    let mut vm = Vm::new(3);
    vm.execute(&[
        Push(100), Store(0),
        Push(200), Store(1),
        Load(0), Load(1), Add, Store(2),
        Load(2), Print,
        Halt,
    ]).unwrap();
    assert_eq!(vm.output(), &[300]);
    println!("  reg[0]+reg[1] = {} stored in reg[2]  ✓", vm.output()[0]);
}

// ── Demo 3: conditional jumps ─────────────────────────────────────────────────
fn demo_control_flow() {
    println!("[ Demo 3 ] conditional control flow");

    // if 5 > 3 { print(1) } else { print(0) }
    // 0: Push(5), Push(3), Gt    — push 1 (true)
    // 3: JumpIfNot(7)            — skip else branch
    // 4: Push(1), Print, Jump(8)
    // 7: Push(0), Print
    // 8: Halt  (index 8 doesn't exist, use Halt at 8)
    let mut vm = Vm::new(0);
    vm.execute(&[
        Push(5), Push(3), Gt,
        JumpIfNot(7),
        Push(1), Print, Jump(8),
        Push(0), Print,
        Halt,
    ]).unwrap();
    assert_eq!(vm.output(), &[1]);
    println!("  5 > 3 → printed {}  ✓", vm.output()[0]);
}

// ── Demo 4: sum 1..=N loop ────────────────────────────────────────────────────
fn demo_sum_loop() {
    println!("[ Demo 4 ] sum 1..=10 via loop");

    // reg[0] = acc = 0
    // reg[1] = i   = 10
    // loop: if i == 0 goto done; acc += i; i -= 1; goto loop
    // done: print acc
    let mut vm = Vm::new(2);
    let program = vec![
        Push(0),  Store(0),   // acc = 0
        Push(10), Store(1),   // i   = 10
        // loop_start = 4
        Load(1), JumpIfNot(15),  // if i == 0 goto 15 (done)
        Load(0), Load(1), Add, Store(0), // acc += i
        Load(1), Push(1),  Sub, Store(1), // i -= 1
        Jump(4),
        // done = 15
        Load(0), Print,
        Halt,
    ];
    vm.execute(&program).unwrap();
    assert_eq!(vm.output(), &[55]); // 1+2+…+10 = 55
    println!("  sum(1..=10) = {}  ✓", vm.output()[0]);
}

// ── Demo 5: Fibonacci(7) = 13 ────────────────────────────────────────────────
//
// reg[0] = a = 0
// reg[1] = b = 1
// reg[2] = counter = n-1 = 6
// loop: tmp = a + b; a = b; b = tmp; counter -= 1; if counter > 0 loop
// print b
fn demo_fibonacci() {
    println!("[ Demo 5 ] Fibonacci(7) = 13");

    let mut vm = Vm::new(4);
    // reg[0]=a, reg[1]=b, reg[2]=counter, reg[3]=tmp
    let program = vec![
        Push(0), Store(0),  // a = 0
        Push(1), Store(1),  // b = 1
        Push(6), Store(2),  // counter = 6 (we want fib(7), starts at 1 so 6 steps)
        // loop_start = 6
        Load(0), Load(1), Add, Store(3), // tmp = a + b
        Load(1), Store(0),               // a = b
        Load(3), Store(1),               // b = tmp
        Load(2), Push(1), Sub, Store(2), // counter -= 1
        Load(2), JumpIf(6),              // if counter != 0 goto loop_start
        Load(1), Print,
        Halt,
    ];
    vm.execute(&program).unwrap();
    assert_eq!(vm.output(), &[13]);
    println!("  fib(7) = {}  ✓", vm.output()[0]);
}
