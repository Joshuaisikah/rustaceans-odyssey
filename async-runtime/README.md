# Async Runtime - Mini Tokio

## 🎯 Learning Objectives

### Core Concepts
- [ ] Future trait and poll mechanism
- [ ] Task scheduling and work stealing
- [ ] Waker and Context internals
- [ ] Pin and self-referential types
- [ ] Reactor pattern (epoll/kqueue)
- [ ] Procedural macros for async syntax
- [ ] Thread pools and CPU affinity

### Book Chapters Covered
- **Chapter 1:** Foundations - Understanding Future
- **Chapter 2:** Types - PhantomData and marker traits
- **Chapter 3:** Designing Interfaces - Executor API
- **Chapter 5:** Project Structure - Modular runtime
- **Chapter 7:** Macros - Custom async/await syntax
- **Chapter 8:** Asynchronous Programming - Deep dive
- **Chapter 12:** Performance - Scheduling optimization

## 🏗️ Project Milestones

### Phase 1: Basic Executor
- [ ] Single-threaded executor
- [ ] Task spawning and scheduling
- [ ] Waker implementation
- [ ] Basic sleep/timer support

### Phase 2: Multi-threaded Runtime
- [ ] Work-stealing scheduler
- [ ] Thread pool management
- [ ] Task migration between threads
- [ ] Load balancing

### Phase 3: I/O Reactor
- [ ] epoll/kqueue integration
- [ ] TcpListener and TcpStream
- [ ] Timer wheel for timeouts
- [ ] Concurrent I/O operations

### Phase 4: Advanced Features
- [ ] LocalSet for !Send futures
- [ ] Blocking task pool
- [ ] Runtime metrics and instrumentation
- [ ] Custom allocators per-task

### Phase 5: Ergonomics
- [ ] Proc macro for #[runtime::main]
- [ ] Proc macro for #[runtime::test]
- [ ] Select! macro for concurrent futures
- [ ] Join! and try_join! macros

## 📚 Required Reading

### Before Starting
- Rust for Rustaceans: Chapters 1-3, 8
- "Asynchronous Programming in Rust" (async book)
- Tokio internals blog series

### During Development
- Withoutboats blog on Pin
- Yoshua Wuyts on executors
- Jon Gjengset's stream on async

## 🎓 Success Criteria

- Pass all async-std compatibility tests
- Handle 10K concurrent tasks
- Single-threaded perf: 90% of tokio
- Multi-threaded perf: 70% of tokio
- Zero unsafe UB (verified with miri)

## 📝 Notes Location
See `NOTES.md` in this directory for learning journal.
