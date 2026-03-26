# VM Runtime - Programming Language Implementation

## 🎯 Learning Objectives

### Core Concepts
- [ ] Bytecode design and execution
- [ ] Stack-based vs register-based VMs
- [ ] Memory management (GC or ownership)
- [ ] Type systems and type inference
- [ ] JIT compilation basics
- [ ] Error handling and panics
- [ ] Foreign function interface
- [ ] Procedural macros for DSLs

### Book Chapters Covered
- **ALL CHAPTERS** - This is your magnum opus

## 🏗️ Project Milestones

### Phase 1: Lexer & Parser
- [ ] Tokenization
- [ ] Recursive descent parser
- [ ] AST representation
- [ ] Syntax error reporting

### Phase 2: Bytecode Compiler
- [ ] AST to bytecode lowering
- [ ] Constant pool
- [ ] Instruction set design (20-30 opcodes)
- [ ] Local variable resolution

### Phase 3: VM Execution
- [ ] Stack-based interpreter
- [ ] Call stack management
- [ ] Closure capture
- [ ] Native functions

### Phase 4: Memory Management
- [ ] Heap allocation for objects
- [ ] Mark-sweep garbage collector
- [ ] OR: Reference counting
- [ ] OR: Arena allocation

### Phase 5: Advanced Features
- [ ] JIT compilation (basic)
- [ ] Tail call optimization
- [ ] First-class functions
- [ ] Module system

### Phase 6: Tooling
- [ ] REPL (Read-Eval-Print-Loop)
- [ ] Debugger with breakpoints
- [ ] Profiler
- [ ] Disassembler

## 📚 Required Reading

### Before Starting
- Rust for Rustaceans: **ALL CHAPTERS**
- "Crafting Interpreters" by Bob Nystrom
- "Engineering a Compiler" (select chapters)

### During Development
- Papers: "A Simple Graph-Based Intermediate Representation"
- Papers: "A Unified Theory of Garbage Collection"
- LLVM tutorials

## 🎓 Success Criteria

- Execute recursive Fibonacci correctly
- Pass 200+ language test cases
- Benchmark: 10-20% of Python speed (interpreted)
- GC never leaks or corrupts memory (tested with valgrind)
- Can run non-trivial programs (1000+ LOC)

## 🧩 Example Language

```rust
// Your language might look like:
fn fibonacci(n) {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

let result = fibonacci(10);
print(result);
```

## 📝 Notes Location
See `NOTES.md` in this directory for learning journal.
