# Database Engine - SQLite Clone

## 🎯 Learning Objectives

### Core Concepts
- [ ] B+ tree implementation
- [ ] Page-based storage management
- [ ] ACID transaction guarantees
- [ ] Query parsing and optimization
- [ ] Type-safe query builders
- [ ] Concurrent access control (MVCC)
- [ ] Write-ahead logging (WAL)

### Book Chapters Covered
- **Chapter 1:** Foundations - Memory-mapped files
- **Chapter 2:** Types - Type-safe schema
- **Chapter 3:** Designing Interfaces - Query builder API
- **Chapter 4:** Error Handling - Transaction errors
- **Chapter 5:** Project Structure - Modular engine
- **Chapter 6:** Testing - Property-based DB tests
- **Chapter 9:** Unsafe Code - Memory-mapped I/O
- **Chapter 10:** Concurrency - MVCC implementation
- **Chapter 11:** FFI - SQL driver interface

## 🏗️ Project Milestones

### Phase 1: Storage Layer
- [ ] Page management (4KB pages)
- [ ] Buffer pool with LRU eviction
- [ ] B+ tree for indexing
- [ ] Heap file for table storage

### Phase 2: Query Engine
- [ ] SQL parser (subset: SELECT, INSERT, UPDATE, DELETE)
- [ ] Query planner and optimizer
- [ ] Executor (volcano model)
- [ ] Index selection

### Phase 3: Transactions
- [ ] ACID guarantees
- [ ] Multi-version concurrency control (MVCC)
- [ ] Snapshot isolation
- [ ] Deadlock detection

### Phase 4: Durability
- [ ] Write-ahead logging (WAL)
- [ ] Crash recovery
- [ ] Checkpointing
- [ ] fsync strategies

### Phase 5: Advanced Features
- [ ] Joins (nested loop, hash join)
- [ ] Aggregations (GROUP BY, COUNT, SUM)
- [ ] Secondary indexes
- [ ] Query caching

## 📚 Required Reading

### Before Starting
- Rust for Rustaceans: Chapters 1-6, 9-11
- "Database Internals" by Alex Petrov
- SQLite architecture documentation

### During Development
- Papers: "ARIES Recovery Algorithm"
- Papers: "B-tree vs LSM-tree"
- CMU Database course lectures

## 🎓 Success Criteria

- Pass SQLite compatibility test suite (subset)
- Handle 1GB database files
- ACID compliant (verified with Jepsen-style tests)
- Query performance: 80% of SQLite
- Zero data corruption under crashes

## 📝 Notes Location
See `NOTES.md` in this directory for learning journal.
