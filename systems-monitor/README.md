# Systems Monitor - htop/prometheus Clone

## 🎯 Learning Objectives

### Core Concepts
- [ ] FFI to system calls (Linux/Windows/macOS)
- [ ] Zero-cost abstractions
- [ ] Performance profiling
- [ ] Plugin architecture with dynamic loading
- [ ] Real-time data processing
- [ ] Cross-platform development
- [ ] Memory-efficient data structures

### Book Chapters Covered
- **Chapter 6:** Testing - Testing across platforms
- **Chapter 9:** Unsafe Code - FFI bindings
- **Chapter 10:** Concurrency - Real-time collection
- **Chapter 11:** Foreign Function Interfaces
- **Chapter 12:** Rust Without the Standard Library

## 🏗️ Project Milestones

### Phase 1: Basic Metrics
- [ ] CPU usage (per-core)
- [ ] Memory usage (RSS, virtual)
- [ ] Process listing
- [ ] Cross-platform abstraction

### Phase 2: Advanced Metrics
- [ ] Disk I/O
- [ ] Network I/O
- [ ] File descriptor usage
- [ ] Context switches

### Phase 3: Real-time Display
- [ ] TUI with ratatui
- [ ] Real-time graphs
- [ ] Process tree view
- [ ] Sorting and filtering

### Phase 4: Plugin System
- [ ] Dynamic library loading
- [ ] Plugin API with C FFI
- [ ] Custom metric plugins
- [ ] Plugin sandboxing

### Phase 5: Export & Integration
- [ ] Prometheus metrics export
- [ ] JSON API
- [ ] WebSocket streaming
- [ ] Alerting system

## 📚 Required Reading

### Before Starting
- Rust for Rustaceans: Chapters 6, 9-12
- Linux `/proc` filesystem documentation
- Windows WMI/Performance Counters docs

### During Development
- Papers: "The Linux Scheduler"
- htop source code
- psutil (Python) for comparison

## 🎓 Success Criteria

- Cross-platform (Linux, macOS, Windows)
- Sub-1% CPU overhead
- Real-time updates at 60 FPS
- Plugin system with 5+ example plugins
- Match htop feature parity

## 📝 Notes Location
See `NOTES.md` in this directory for learning journal.
