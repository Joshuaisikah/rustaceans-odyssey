// ─── vm-runtime ──────────────────────────────────────────────────────────────
//
// A stack-based bytecode virtual machine.
//
// Module layout:
//   opcode — Opcode enum (instruction set) + VmError
//   vm     — Vm (stack, registers, output log, execute loop)

pub mod opcode;
pub mod vm;

pub use opcode::{Opcode, VmError};
pub use vm::Vm;
