# Quick Reference - Keep This Open While Coding

## 🔥 Before Writing ANY Code

```markdown
1. What invariants must hold?
2. What can go wrong?
3. What are the performance characteristics?
4. Can the type system enforce this?
```

## ✅ Code Review Checklist (Use This EVERY Time)

```rust
// Before committing:
- [ ] No unwrap() without proof
- [ ] Every unsafe has SAFETY comment
- [ ] Lifetimes are descriptive ('conn not 'a)
- [ ] Benchmarked if performance-critical
- [ ] All errors have context
- [ ] Public APIs have examples in docs
- [ ] Tests cover edge cases
```

## 🎯 Quality Mantras

> **Make it work, make it right, make it fast.**
> **Measure, don't guess.**
> **Type systems are proof systems.**

## 📝 Template: Unsafe Code

```rust
// SAFETY: [Explain why this is safe]
// 1. Pointer is valid because...
// 2. No aliasing because...
// 3. Lifetime is bounded by...
unsafe {
    // minimal unsafe code here
}
```

## 📝 Template: Public API

```rust
/// [What it does - one sentence]
///
/// [When to use it vs alternatives]
///
/// # Examples
///
/// ```
/// # use crate::*;
/// let x = api_method();
/// assert_eq!(x, expected);
/// ```
///
/// # Panics
///
/// Panics if [condition]
///
/// # Errors
///
/// Returns `Err` if [condition]
pub fn api_method(&self) -> Result<T, Error> {
    todo!()
}
```

## 🚫 Common Mistakes to Avoid

```rust
// ❌ BAD
let x = value.unwrap();  // Can panic!
let _ = result;  // Silent failure!
'a, 'b, 'c  // Meaningless lifetimes
unsafe { ... }  // No safety comment!

// ✅ GOOD
let x = value?;  // Propagate error
let _ignored = result;  // Intentional with name
'conn, 'arena, 'static  // Descriptive
// SAFETY: ptr is valid because...
unsafe { ... }
```

## ⚡ Performance Quick Wins

```rust
// Allocation awareness
String::with_capacity(n)  // Pre-allocate
Vec::with_capacity(n)     // Pre-allocate

// Borrowing over cloning
fn process(data: &[u8])   // Borrow
fn process(data: Vec<u8>) // Only if you need ownership

// Iterator chains
.iter().map().filter().collect()  // Single pass
```

## 🧪 Testing Commands

```bash
# Run these frequently
cargo test
cargo clippy -- -D warnings
cargo bench --bench my_bench
cargo miri test  # For unsafe code
cargo fuzz run target  # For parsers
```

## 📊 Profiling Commands

```bash
# CPU profiling
cargo flamegraph --bin mybinary

# Memory profiling
cargo build --release
valgrind --tool=massif ./target/release/mybinary

# Allocation profiling
RUSTFLAGS="-C force-frame-pointers=yes" cargo build --release
heaptrack ./target/release/mybinary
```

## 🎓 When Stuck

1. Read the relevant chapter in "Rust for Rustaceans"
2. Check stdlib docs for similar patterns
3. Look at how tokio/serde/etc do it
4. Write notes explaining it to yourself
5. Code ONLY after understanding

## 💾 Save This Workflow

```markdown
1. Read chapter
2. Take notes
3. Plan code (document invariants)
4. Write tests first
5. Implement
6. Benchmark/profile
7. Document learnings in NOTES.md
8. Move to next feature
```

---

**Print this. Keep it visible. Reference it constantly.**
