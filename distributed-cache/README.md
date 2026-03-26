# Distributed Cache - Redis/Memcached Clone

## 🎯 Learning Objectives

### Core Concepts
- [ ] Custom memory allocators (arena, slab)
- [ ] Lock-free data structures (concurrent hashmap)
- [ ] Async I/O patterns (tokio runtime)
- [ ] Wire protocol design (RESP-like protocol)
- [ ] Zero-copy operations with unsafe
- [ ] FFI bindings (Python/Node.js clients)

### Book Chapters Covered
- **Chapter 1:** Foundations - Memory layout, unsafe basics
- **Chapter 2:** Types - Builder patterns for client API
- **Chapter 3:** Designing Interfaces - Public API design
- **Chapter 4:** Error Handling - Network errors, timeouts
- **Chapter 9:** Unsafe Code - Raw pointers for zero-copy
- **Chapter 10:** Concurrency - Lock-free structures
- **Chapter 11:** FFI - Language bindings

## 🏗️ Project Milestones

### Phase 1: Core Storage
- [ ] In-memory hashmap with TTL support
- [ ] Custom allocator for value storage
- [ ] Basic SET/GET/DEL operations
- [ ] Memory limits and eviction (LRU)

### Phase 2: Concurrency
- [ ] Lock-free concurrent hashmap
- [ ] Multi-threaded request handling
- [ ] Benchmark: 100K ops/sec single-threaded
- [ ] Benchmark: 500K ops/sec multi-threaded

### Phase 3: Networking
- [ ] TCP server with async I/O
- [ ] Custom wire protocol (binary)
- [ ] Connection pooling
- [ ] Pipelining support

### Phase 4: Advanced Features
- [ ] Pub/Sub messaging
- [ ] Persistence (AOF/RDB snapshots)
- [ ] Replication (leader-follower)
- [ ] Cluster mode (sharding)

### Phase 5: Language Bindings
- [ ] C FFI interface
- [ ] Python client (PyO3)
- [ ] Node.js client (napi-rs)

## 📚 Required Reading

### Before Starting
- Rust for Rustaceans: Chapters 1-4
- "Designing Data-Intensive Applications" - Caching chapter

### During Development
- Papers: "Lock-Free Data Structures"
- Redis protocol specification
- Memcached architecture

## 🎓 Success Criteria

- Pass all property-based tests (proptest)
- Handle 1M concurrent connections
- Sub-microsecond latency for GET operations
- Zero data races (verified with loom)
- FFI clients work from 3+ languages

## 📝 Notes Location
See `NOTES.md` in this directory for learning journal.
