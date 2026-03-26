# Rustaceans Odyssey 🦀

**A journey through advanced Rust concepts by building 5 production-grade systems**

> "The only way to truly master Rust is to build something that makes you question everything you know about programming."

## 📚 Learning Resources

This journey uses **two complementary books**:

- **Programming Rust** (O'Reilly) - Comprehensive foundation & reference
- **Rust for Rustaceans** (No Starch) - Advanced patterns & production practices

See `BOOKS_GUIDE.md` for complete reading roadmap and how to use both books together.

## 🎯 Mission
Build **5 complete, production-quality projects** that will force you to:
- Read and re-read "Rust for Rustaceans"
- Take extensive notes on advanced concepts
- Understand the *why* behind Rust's design decisions
- Master unsafe code, concurrency, type systems, and performance optimization

## 📚 The Five Pillars

### 1. **Distributed Cache** (`distributed-cache/`)
**Concepts Mastered:**
- Custom memory allocators & arena allocation
- Lock-free concurrent data structures
- Async I/O patterns
- Network protocols & serialization
- FFI for language bindings
- Unsafe code for zero-copy operations

**Chapters:** 1-4, 9-11

---

### 2. **Async Runtime** (`async-runtime/`)
**Concepts Mastered:**
- Custom task schedulers & executors
- Advanced lifetimes & phantom types
- Pinning and self-referential structures
- Procedural macros for ergonomic APIs
- Thread pools & work stealing
- Future trait internals

**Chapters:** 1-3, 5, 7-8, 12

---

### 3. **Database Engine** (`database-engine/`)
**Concepts Mastered:**
- B-tree/LSM tree implementations
- Transaction management & ACID guarantees
- Query parsing with macros
- Buffer pool management
- Concurrent access patterns
- Type-safe query builders

**Chapters:** 1-6, 9-11

---

### 4. **VM Runtime** (`vm-runtime/`)
**Concepts Mastered:**
- Bytecode VM & stack-based execution
- Memory models & garbage collection
- JIT compilation basics
- Error handling & panics
- Plugin systems with dynamic loading
- DSL creation with macros

**Chapters:** ALL (1-12)

---

### 5. **Systems Monitor** (`systems-monitor/`)
**Concepts Mastered:**
- FFI to system calls (Linux/Windows)
- Zero-cost abstractions
- Performance profiling & benchmarking
- Plugin architecture
- Real-time data processing
- Cross-platform development

**Chapters:** 6, 9-12

---

## 📖 Study Guide

### Phase 1: Foundations (Projects 1 & 5)
Start with **Distributed Cache** and **Systems Monitor** simultaneously
- These introduce you to unsafe code, FFI, and performance
- Build muscle memory for reading docs and taking notes

### Phase 2: Advanced Types (Projects 2 & 3)
Move to **Async Runtime** and **Database Engine**
- Deep dive into type systems, lifetimes, and trait design
- Master concurrent data structures

### Phase 3: Integration (Project 4)
Finish with **VM Runtime**
- Combines ALL concepts from previous projects
- Your magnum opus

---

## 📝 Notes Structure
Each project has a `NOTES.md` file tracking:
- Concepts learned
- Book chapters referenced
- Aha! moments
- Questions & research needed
- Performance optimizations tried

See `notes/` directory for cross-cutting research topics.

---

## 🔥 Learning Rules

1. **No copy-paste from tutorials** - Understand every line
2. **Write notes BEFORE writing code** - Plan your approach
3. **Read the book chapter first** - Then apply to project
4. **If you don't understand it, you don't write it** - Research until clear
5. **Test everything** - Write tests that teach you how it works
6. **Benchmark everything** - Prove your optimizations work

---

## 🚀 Getting Started

```bash
# Build everything
cargo build --workspace

# Test everything
cargo test --workspace

# Work on a specific project
cd distributed-cache
cargo watch -x test -x run
```

---

## 📚 Resources Beyond the Book

- The Rustonomicon (unsafe code)
- Rust Performance Book
- Jon Gjengset's YouTube channel
- Database Internals (Alex Petrov)
- Crafting Interpreters (Bob Nystrom)
- LLVM documentation

---

## 🎓 Progress Tracking

- [ ] Distributed Cache - Chapter 1: Foundations
- [ ] Distributed Cache - Chapter 2: Types
- [ ] Distributed Cache - Chapter 3: Lifetimes
- [ ] ... (continue for each chapter/project)

---

**Start Date:** 2026-03-26

**Estimated Completion:** When you can explain every line of code to someone else

**Success Metric:** When you dream in Rust lifetimes

---

*"The struggle itself toward the heights is enough to fill a man's heart." - Albert Camus*
