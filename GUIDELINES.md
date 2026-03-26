# Professional Rust Development Guidelines

## 🎯 Core Principles

### 1. **Correctness First, Performance Second**
```rust
// ❌ BAD: Fast but unsafe
unsafe { *ptr = value }  // No validation

// ✅ GOOD: Safe first, then optimize
let slot = unsafe { &mut *ptr };
debug_assert!(is_valid_ptr(ptr));
*slot = value;
```

**Rule:** Always write the safe version first. Profile. Then optimize with unsafe if needed.

---

### 2. **Make Invalid States Unrepresentable**
```rust
// ❌ BAD: Can represent invalid state
struct Connection {
    socket: Option<TcpStream>,
    is_connected: bool,  // Can be out of sync!
}

// ✅ GOOD: Type system enforces correctness
enum Connection {
    Connected(TcpStream),
    Disconnected,
}
```

**Rule:** Use enums and the type system to prevent bugs at compile time.

---

### 3. **Document Your Invariants**
```rust
/// A non-empty vector that maintains sorted order.
///
/// # Invariants
/// - Length is always >= 1
/// - Elements are sorted in ascending order
/// - No duplicates allowed
///
/// # Safety
/// Methods assume these invariants hold. Breaking them is UB.
pub struct SortedVec<T> {
    inner: Vec<T>,
}
```

**Rule:** Every unsafe block, every custom type, document the invariants.

---

### 4. **Prefer Compile-Time Over Runtime**
```rust
// ❌ BAD: Runtime check
fn process(size: usize) {
    assert!(size <= 1024);
    // ...
}

// ✅ GOOD: Compile-time guarantee
use typenum::U1024;
fn process<N: Unsigned + IsLessOrEqual<U1024>>() {
    // Size checked at compile time
}
```

**Rule:** Push as many checks as possible to compile time using const generics and trait bounds.

---

### 5. **Zero-Cost Abstractions Are Not Free**
```rust
// ❌ BAD: Abstraction without measurement
trait Handler {
    fn handle(&self, event: Event);
}
// Used in hot path, causes dynamic dispatch overhead

// ✅ GOOD: Measured first
// Profiled: dynamic dispatch costs 15% in hot path
// Solution: Use generics or enum dispatch
enum Handler {
    FastPath(FastHandler),
    SlowPath(SlowHandler),
}
```

**Rule:** Measure before optimizing. "Zero-cost" means no runtime overhead, not no compile-time complexity.

---

## 📝 Code Quality Checklist

Before committing code, verify:

### Type Safety
- [ ] No `unwrap()` without proof it can't panic
- [ ] No `unsafe` without SAFETY comment explaining invariants
- [ ] Enums represent all valid states, no impossible combinations
- [ ] Lifetimes are explicit and meaningful (no `'a`, `'b` - use `'arena`, `'request`)

### Performance
- [ ] Benchmarked hot paths (use `cargo bench`)
- [ ] Profiled allocations (use `dhat` or `heaptrack`)
- [ ] Zero unnecessary copies (use `#[derive(Clone)]` sparingly)
- [ ] Considered cache locality for data structures

### Error Handling
- [ ] All errors have context (use `anyhow::Context` or `thiserror`)
- [ ] No silent failures (`let _ = ...` requires comment)
- [ ] Recovery path for every error
- [ ] User-facing errors are actionable

### Documentation
- [ ] Public items have doc comments with examples
- [ ] Unsafe code has SAFETY explanations
- [ ] Complex algorithms reference papers/sources
- [ ] Panic conditions documented

### Testing
- [ ] Unit tests for each module
- [ ] Property-based tests for complex logic (use `proptest`)
- [ ] Fuzzing for parsers/deserializers (use `cargo-fuzz`)
- [ ] Integration tests for public APIs

---

## 🧠 Thinking Framework

### Before Writing Code

**Ask yourself:**

1. **What invariants must hold?**
   - Write them down first
   - Can the type system enforce them?

2. **What can go wrong?**
   - List all error cases
   - Design error types before happy path

3. **What are the performance characteristics?**
   - Time complexity: O(?)
   - Space complexity: O(?)
   - Allocation pattern: stack/heap/arena?

4. **What will future-you need to know?**
   - Document the *why*, not the *what*
   - Link to resources you read

### While Writing Code

**Pause and verify:**

1. **Every `unsafe` block**
   ```rust
   // SAFETY: [Explain why this is safe]
   // 1. Pointer is valid because...
   // 2. No aliasing because...
   // 3. Lifetime is bounded by...
   unsafe { ... }
   ```

2. **Every `.unwrap()` or `.expect()`**
   ```rust
   // Why can't this fail?
   // If it can fail, use `?` or handle it
   let value = option.expect("Config validated at startup");
   ```

3. **Every public API**
   ```rust
   /// What does this do?
   /// When should I use it vs alternatives?
   /// What are the edge cases?
   /// # Examples
   /// # Panics
   /// # Errors
   pub fn api_method(&self) -> Result<T> { ... }
   ```

### After Writing Code

**Review:**

1. **Run all checks**
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt -- --check
   cargo doc --no-deps --open
   ```

2. **Benchmark if performance-critical**
   ```bash
   cargo bench --bench my_bench
   ```

3. **Ask: Can I delete code?**
   - Simpler is better
   - Every line is a liability

---

## 🔬 Research Protocol

When you don't understand something:

### 1. **Check the Book**
- "Rust for Rustaceans" - chapter?
- The Rust Book - fundamentals
- Rustonomicon - unsafe code

### 2. **Read the Docs**
- `std` documentation
- Crate docs on docs.rs
- API design rationale

### 3. **Study the Source**
- Look at stdlib implementation
- Read popular crates (tokio, serde, etc.)
- Understand *why* they made choices

### 4. **Document Your Learning**
```markdown
## Date: 2026-03-26
### Topic: Pin and Unpin

**Question:** Why does async need Pin?

**Research:**
- Read: Rust for Rustaceans Ch. 8
- Read: Pin documentation
- Read: Tokio internals blog post

**Answer:**
[Your understanding in your own words]

**Example:**
[Code demonstrating the concept]

**Questions Remaining:**
- How does Pin interact with PhantomPinned?
```

---

## 🎨 Code Style

### Naming Conventions
```rust
// Types: PascalCase, descriptive nouns
struct HttpClient { }
enum ConnectionState { }

// Functions: snake_case, verb phrases
fn parse_request() { }
fn is_valid() -> bool { }

// Constants: SCREAMING_SNAKE_CASE
const MAX_BUFFER_SIZE: usize = 1024;

// Lifetimes: descriptive
'conn not 'a
'arena not 'b
'static is the only exception
```

### Module Organization
```
src/
├── lib.rs              # Public API only
├── types.rs            # Core type definitions
├── error.rs            # Error types
├── parser/
│   ├── mod.rs          # Parser public API
│   ├── lexer.rs        # Implementation detail
│   └── ast.rs          # Implementation detail
└── runtime/
    ├── mod.rs
    └── executor.rs
```

### Import Style
```rust
// Stdlib first, alphabetical
use std::collections::HashMap;
use std::sync::Arc;

// External crates, alphabetical
use anyhow::Result;
use serde::{Deserialize, Serialize};

// Local modules, explicit
use crate::error::Error;
use crate::types::{Request, Response};
```

---

## ⚡ Performance Guidelines

### Allocation Awareness
```rust
// ❌ BAD: Hidden allocations
let s = format!("value: {}", x);  // Allocates always

// ✅ GOOD: Avoid allocation when possible
use std::fmt::Write;
let mut s = String::with_capacity(estimated_size);
write!(&mut s, "value: {}", x)?;
```

### Borrowing Over Cloning
```rust
// ❌ BAD: Unnecessary clone
fn process(data: Vec<u8>) -> Vec<u8> {
    data.clone()  // Why?
}

// ✅ GOOD: Borrow when possible
fn process(data: &[u8]) -> Vec<u8> {
    data.to_vec()  // Only when needed
}
```

### Iterator Efficiency
```rust
// ❌ BAD: Multiple iterations
let vec: Vec<_> = data.iter().map(|x| x * 2).collect();
let sum: i32 = vec.iter().sum();

// ✅ GOOD: Single pass
let (vec, sum) = data.iter()
    .map(|x| x * 2)
    .fold((Vec::new(), 0), |(mut v, sum), x| {
        v.push(x);
        (v, sum + x)
    });
```

---

## 🛡️ Safety Guidelines

### Unsafe Code Rules

1. **Minimize unsafe surface area**
   ```rust
   // ✅ GOOD: Unsafe confined to small function
   fn get_unchecked(&self, idx: usize) -> &T {
       debug_assert!(idx < self.len);
       // SAFETY: Index bounds checked above
       unsafe { self.data.get_unchecked(idx) }
   }
   ```

2. **Document all preconditions**
   ```rust
   /// # Safety
   /// - `ptr` must be valid for reads
   /// - `ptr` must be properly aligned
   /// - `ptr` must point to initialized T
   unsafe fn read_ptr<T>(ptr: *const T) -> T { ... }
   ```

3. **Use debug assertions**
   ```rust
   // Catch bugs in debug, optimize in release
   debug_assert!(idx < len);
   unsafe { ... }
   ```

---

## 📊 When to Optimize

```
1. Write clear, correct code
2. Measure with benchmarks
3. Profile to find bottlenecks
4. Optimize the bottleneck
5. Measure again
6. Document the optimization
```

**Never optimize without profiling first.**

---

## 🎓 Learning Mantras

> **"If you can't explain it simply, you don't understand it well enough."**
>
> Write notes as if teaching someone else.

> **"Make it work, make it right, make it fast."**
>
> In that order. Always.

> **"The best code is no code."**
>
> Delete before you add. Simplify before you complicate.

> **"Measure, don't guess."**
>
> Benchmarks over intuition. Profiling over assumptions.

> **"Type systems are proof systems."**
>
> Use types to prove correctness at compile time.

---

## 🔍 Code Review Questions

Before considering code "done", ask:

- [ ] Can this panic? Have I handled all error cases?
- [ ] Is this the simplest solution that works?
- [ ] Have I documented why, not just what?
- [ ] Would I understand this code in 6 months?
- [ ] Have I tested edge cases?
- [ ] Is this performant enough? Have I measured?
- [ ] Does this follow Rust idioms?
- [ ] Can the type system enforce this better?

---

**Remember:** The goal isn't just working code. It's *understanding*.

Every line you write should teach you something about Rust.
