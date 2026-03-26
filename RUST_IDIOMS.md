# Rust Idioms & Patterns - Your Daily Reference

## 🎯 Essential Patterns You'll Use Constantly

---

## 1. The Newtype Pattern

**Use when:** You want type safety for primitives

```rust
// ❌ BAD: Easy to mix up
fn transfer(from: u64, to: u64, amount: u64) { }
transfer(amount, from, to);  // Oops! Wrong order

// ✅ GOOD: Compile-time safety
struct AccountId(u64);
struct Amount(u64);

fn transfer(from: AccountId, to: AccountId, amount: Amount) { }
// transfer(amount, from, to);  // Won't compile!
```

**Benefits:**
- Zero runtime cost
- Prevents accidental mixing
- Self-documenting code

---

## 2. Builder Pattern

**Use when:** Struct has many optional fields

```rust
// ❌ BAD: Too many constructors
impl Cache {
    fn new() -> Self { }
    fn new_with_capacity(cap: usize) -> Self { }
    fn new_with_ttl(ttl: Duration) -> Self { }
    fn new_with_capacity_and_ttl(cap: usize, ttl: Duration) -> Self { }
    // Combinatorial explosion!
}

// ✅ GOOD: Builder pattern
struct CacheBuilder {
    capacity: Option<usize>,
    default_ttl: Option<Duration>,
    eviction_policy: EvictionPolicy,
}

impl CacheBuilder {
    fn new() -> Self {
        Self {
            capacity: None,
            default_ttl: None,
            eviction_policy: EvictionPolicy::LRU,
        }
    }

    fn capacity(mut self, cap: usize) -> Self {
        self.capacity = Some(cap);
        self
    }

    fn ttl(mut self, ttl: Duration) -> Self {
        self.default_ttl = Some(ttl);
        self
    }

    fn build(self) -> Cache {
        Cache {
            capacity: self.capacity.unwrap_or(1024),
            default_ttl: self.default_ttl.unwrap_or(Duration::from_secs(3600)),
            eviction_policy: self.eviction_policy,
            // ...
        }
    }
}

// Usage
let cache = CacheBuilder::new()
    .capacity(2048)
    .ttl(Duration::from_secs(600))
    .build();
```

---

## 3. RAII (Resource Acquisition Is Initialization)

**Use when:** Managing resources that need cleanup

```rust
// ✅ Lock automatically released when guard drops
fn update_counter(counter: &Mutex<i32>) {
    let mut guard = counter.lock().unwrap();
    *guard += 1;
    // Lock automatically released here
}

// ✅ File automatically closed
fn write_log(msg: &str) -> io::Result<()> {
    let mut file = File::create("log.txt")?;
    file.write_all(msg.as_bytes())?;
    // File closed automatically
    Ok(())
}

// ✅ Custom RAII guard
struct Transaction<'db> {
    db: &'db mut Database,
    committed: bool,
}

impl<'db> Transaction<'db> {
    fn commit(mut self) {
        self.db.commit();
        self.committed = true;
    }
}

impl Drop for Transaction<'_> {
    fn drop(&mut self) {
        if !self.committed {
            self.db.rollback();  // Auto-rollback if not committed
        }
    }
}
```

---

## 4. Type State Pattern

**Use when:** Object behavior changes based on state

```rust
// ✅ Compile-time state machine
struct Locked;
struct Unlocked;

struct Door<State> {
    _state: PhantomData<State>,
}

impl Door<Locked> {
    fn new() -> Self {
        Door { _state: PhantomData }
    }

    fn unlock(self, key: &Key) -> Door<Unlocked> {
        // Verify key...
        Door { _state: PhantomData }
    }
}

impl Door<Unlocked> {
    fn open(&self) {
        println!("Door is opening");
    }

    fn lock(self) -> Door<Locked> {
        Door { _state: PhantomData }
    }
}

// Usage
let door = Door::new();  // Door<Locked>
// door.open();  // Won't compile! Door is locked
let door = door.unlock(&key);  // Door<Unlocked>
door.open();  // OK!
```

---

## 5. Extension Trait Pattern

**Use when:** Adding methods to types you don't own

```rust
// Add methods to slices
trait SliceExt<T> {
    fn second(&self) -> Option<&T>;
}

impl<T> SliceExt<T> for [T] {
    fn second(&self) -> Option<&T> {
        self.get(1)
    }
}

// Usage
let nums = vec![1, 2, 3];
assert_eq!(nums.second(), Some(&2));
```

---

## 6. Visitor Pattern (with Traits)

**Use when:** Separating algorithms from data structures

```rust
trait Visitor {
    fn visit_number(&mut self, n: i32);
    fn visit_string(&mut self, s: &str);
}

enum Value {
    Number(i32),
    String(String),
}

impl Value {
    fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Value::Number(n) => visitor.visit_number(*n),
            Value::String(s) => visitor.visit_string(s),
        }
    }
}

struct PrintVisitor;

impl Visitor for PrintVisitor {
    fn visit_number(&mut self, n: i32) {
        println!("Number: {}", n);
    }
    fn visit_string(&mut self, s: &str) {
        println!("String: {}", s);
    }
}
```

---

## 7. Cow (Clone on Write)

**Use when:** Want to avoid clones unless necessary

```rust
use std::borrow::Cow;

fn process_name(name: Cow<str>) -> Cow<str> {
    if name.starts_with("Dr. ") {
        name  // Return borrowed, no allocation
    } else {
        Cow::Owned(format!("Dr. {}", name))  // Clone only when needed
    }
}

// Usage
let name = "Smith";
let result = process_name(Cow::Borrowed(name));  // No allocation

let name = "Dr. Jones";
let result = process_name(Cow::Borrowed(name));  // Still no allocation!
```

---

## 8. Interior Mutability

**Use when:** Need mutation through shared reference

```rust
use std::cell::RefCell;

// Single-threaded interior mutability
struct Cache {
    data: RefCell<HashMap<String, String>>,
}

impl Cache {
    fn get(&self, key: &str) -> Option<String> {
        // Can mutate through &self!
        self.data.borrow().get(key).cloned()
    }

    fn insert(&self, key: String, value: String) {
        self.data.borrow_mut().insert(key, value);
    }
}

// Multi-threaded: use Mutex or RwLock
struct ThreadSafeCache {
    data: RwLock<HashMap<String, String>>,
}
```

---

## 9. Error Handling Patterns

### Pattern A: Use `?` for propagation

```rust
fn read_config() -> Result<Config, Error> {
    let file = File::open("config.toml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config = toml::from_str(&contents)?;
    Ok(config)
}
```

### Pattern B: Context for better errors

```rust
use anyhow::{Context, Result};

fn load_user(id: u64) -> Result<User> {
    let file = File::open(format!("users/{}.json", id))
        .context("Failed to open user file")?;

    let user = serde_json::from_reader(file)
        .with_context(|| format!("Failed to parse user {}", id))?;

    Ok(user)
}
```

### Pattern C: Custom error types

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum CacheError {
    #[error("Key not found: {0}")]
    KeyNotFound(String),

    #[error("Cache is full (max: {max}, current: {current})")]
    CacheFull { max: usize, current: usize },

    #[error("IO error")]
    Io(#[from] std::io::Error),
}
```

---

## 10. Iterator Patterns

### Pattern A: Adapter chains

```rust
// ✅ Efficient: single pass, lazy evaluation
let result: Vec<_> = data
    .iter()
    .filter(|x| x.is_valid())
    .map(|x| x.process())
    .take(10)
    .collect();
```

### Pattern B: Custom iterators

```rust
struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        let result = self.curr;
        self.curr = self.next;
        self.next = new_next;
        Some(result)
    }
}

// Usage
let fib = Fibonacci { curr: 0, next: 1 };
for num in fib.take(10) {
    println!("{}", num);
}
```

---

## 11. Smart Pointer Patterns

### When to use each:

```rust
// Box: Heap allocation, single owner
let b: Box<i32> = Box::new(5);

// Rc: Multiple owners, single-threaded
let a = Rc::new(vec![1, 2, 3]);
let b = Rc::clone(&a);  // Both own the data

// Arc: Multiple owners, multi-threaded
let data = Arc::new(Mutex::new(0));
let data_clone = Arc::clone(&data);
thread::spawn(move || {
    *data_clone.lock().unwrap() += 1;
});

// RefCell: Interior mutability, single-threaded
let value = RefCell::new(5);
*value.borrow_mut() += 1;
```

---

## 12. Lifetime Elision Rules

```rust
// Rule 1: Each elided lifetime gets unique parameter
fn first(s: &str) -> &str { }  // Expands to:
fn first<'a>(s: &'a str) -> &'a str { }

// Rule 2: If one input lifetime, assigned to output
fn take_first<'a>(x: &'a str, y: &str) -> &'a str { }

// Rule 3: If &self, lifetime of self assigned to output
impl Foo {
    fn get(&self) -> &str { }  // Returns reference to self
}
```

---

## 13. Deref Coercion

**Use when:** Want smart pointer to act like contained type

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Now MyBox<String> can be used like &str
fn print_str(s: &str) {
    println!("{}", s);
}

let b = MyBox(String::from("hello"));
print_str(&b);  // Deref coercion: MyBox -> String -> &str
```

---

## 14. From/Into for Conversions

```rust
// Implement From, get Into for free
struct UserId(u64);

impl From<u64> for UserId {
    fn from(id: u64) -> Self {
        UserId(id)
    }
}

// Usage
let id: UserId = 42u64.into();  // Into comes free!
let id = UserId::from(42u64);   // From is what you impl'd
```

---

## 15. AsRef/AsMut for Flexibility

```rust
// Accept anything that can be viewed as &str
fn print_it<S: AsRef<str>>(s: S) {
    println!("{}", s.as_ref());
}

// Works with String, &str, Cow<str>, etc.
print_it("hello");
print_it(String::from("world"));
print_it(&my_string);
```

---

## 16. Unsafe Guidelines

### Always document with SAFETY comment

```rust
/// # Safety
///
/// `ptr` must:
/// - Be valid for reads
/// - Be properly aligned
/// - Point to initialized T
unsafe fn read_unchecked<T>(ptr: *const T) -> T {
    // SAFETY: Caller guarantees ptr is valid, aligned, and initialized
    unsafe { ptr.read() }
}
```

### Minimize unsafe scope

```rust
// ❌ BAD: Large unsafe block
unsafe {
    let a = some_unsafe_operation();
    let b = safe_operation(a);
    let c = another_unsafe_operation(b);
    c
}

// ✅ GOOD: Minimal unsafe blocks
let a = unsafe { some_unsafe_operation() };
let b = safe_operation(a);
let c = unsafe { another_unsafe_operation(b) };
```

---

## 17. Macro Patterns

### Declarative macros

```rust
macro_rules! hashmap {
    ($($key:expr => $val:expr),* $(,)?) => {{
        let mut map = HashMap::new();
        $(
            map.insert($key, $val);
        )*
        map
    }};
}

// Usage
let map = hashmap! {
    "a" => 1,
    "b" => 2,
};
```

---

## 18. Trait Object vs Generics

```rust
// Static dispatch (monomorphization)
fn process<T: Handler>(handler: T) {
    handler.handle();
}
// Pro: Fast (inlined)
// Con: Code bloat if many types

// Dynamic dispatch (trait objects)
fn process(handler: &dyn Handler) {
    handler.handle();
}
// Pro: Smaller code size
// Con: Virtual call overhead (small)
```

---

## 19. Const Generics

```rust
// Type-safe fixed-size arrays
struct Matrix<T, const ROWS: usize, const COLS: usize> {
    data: [[T; COLS]; ROWS],
}

impl<T, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    fn new(data: [[T; COLS]; ROWS]) -> Self {
        Matrix { data }
    }
}

// Usage
let m: Matrix<i32, 3, 3> = Matrix::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
```

---

## 20. Pin for Self-Referential Structs

```rust
use std::pin::Pin;

struct SelfRef {
    data: String,
    ptr: *const String,  // Points to self.data
}

impl SelfRef {
    fn new(s: String) -> Pin<Box<Self>> {
        let mut boxed = Box::pin(SelfRef {
            data: s,
            ptr: std::ptr::null(),
        });

        // Safe to create self-reference after pinning
        let ptr = &boxed.data as *const String;
        unsafe {
            let mut_ref = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).ptr = ptr;
        }

        boxed
    }
}
```

---

## 🎓 When to Use What?

| Pattern | Use When |
|---------|----------|
| Newtype | Need type safety for primitives |
| Builder | Many optional parameters |
| RAII | Managing resources (files, locks, connections) |
| Type State | State machine with compile-time checks |
| Extension Trait | Adding methods to external types |
| Cow | Avoiding clones in read-heavy scenarios |
| Interior Mutability | Need mutation through `&self` |
| Custom Error Types | Building libraries with specific errors |
| Trait Objects | Runtime polymorphism needed |
| Generics | Performance critical, static dispatch |

---

## 📚 Remember

> **"Good Rust code uses the type system to prove correctness."**
>
> **"If it compiles, it usually works."**
>
> **"Fight the borrow checker early, become friends later."**

---

**Keep this reference handy. These patterns will become second nature.**

**Your future self will thank you for using them correctly!** 🦀
