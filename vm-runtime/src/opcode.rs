// ─── Instruction set ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    // Stack manipulation
    Push(i64), // push literal
    Pop,       // discard top
    Dup,       // duplicate top
    Swap,      // swap top two

    // Arithmetic (pop two operands, push result: lhs op rhs where lhs is deeper)
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg, // negate top (unary)

    // Comparison (pop two, push 1 for true / 0 for false)
    Eq, Ne,
    Lt, Le,
    Gt, Ge,

    // Control flow (jump target = instruction index)
    Jump(usize),      // unconditional
    JumpIf(usize),    // pop; jump if non-zero
    JumpIfNot(usize), // pop; jump if zero

    // Registers
    Load(usize),  // push register[n] onto stack
    Store(usize), // pop top into register[n]

    // I/O
    Print, // pop top and record in output log

    // Termination
    Halt,
}

// ─── Errors ──────────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq)]
pub enum VmError {
    StackUnderflow,
    DivisionByZero,
    InvalidJump     { target: usize, len: usize },
    InvalidRegister { index: usize },
}

impl std::fmt::Display for VmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VmError::StackUnderflow          => write!(f, "stack underflow"),
            VmError::DivisionByZero          => write!(f, "division by zero"),
            VmError::InvalidJump { target, len } =>
                write!(f, "invalid jump to {target} (program length {len})"),
            VmError::InvalidRegister { index } =>
                write!(f, "invalid register index {index}"),
        }
    }
}

impl std::error::Error for VmError {}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // VmError Display must not panic.
    #[test]
    fn test_vm_error_display() {
        assert!(!VmError::StackUnderflow.to_string().is_empty());
        assert!(!VmError::DivisionByZero.to_string().is_empty());
        assert!(!VmError::InvalidJump { target: 5, len: 3 }.to_string().is_empty());
        assert!(!VmError::InvalidRegister { index: 99 }.to_string().is_empty());
    }

    // Opcode derives PartialEq for use in assertions.
    #[test]
    fn test_opcode_equality() {
        assert_eq!(Opcode::Push(42), Opcode::Push(42));
        assert_ne!(Opcode::Push(1),  Opcode::Push(2));
        assert_eq!(Opcode::Halt,     Opcode::Halt);
        assert_ne!(Opcode::Add,      Opcode::Sub);
    }

    // Jump targets are stored correctly.
    #[test]
    fn test_jump_opcode_stores_target() {
        assert_eq!(Opcode::Jump(10),      Opcode::Jump(10));
        assert_eq!(Opcode::JumpIf(5),     Opcode::JumpIf(5));
        assert_eq!(Opcode::JumpIfNot(0),  Opcode::JumpIfNot(0));
    }
}
