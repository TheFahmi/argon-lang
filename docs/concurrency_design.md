# Cryo Concurrency Design (True Multithreading)

## The Challenge
Current architecture uses `Rc<RefCell<T>>` which is strictly single-threaded.
To enable `spawn(|| { ... })`, we need to change the core memory model.

## Architecture Migration

### 1. Value Type Definition
**Current (Single Threaded):**
```rust
pub enum Value {
    Array(Rc<RefCell<Vec<Value>>>), // Fast, but unsafe across threads
    Struct(Rc<RefCell<HashMap<String, Value>>>),
}
```

**Proposed (Thread Safe):**
```rust
pub enum Value {
    // Arc = Atomic Reference Counting (Thread safe)
    // RwLock = Read-Write Lock (Allow multiple readers or one writer)
    Array(Arc<RwLock<Vec<Value>>>), 
    Struct(Arc<RwLock<HashMap<String, Value>>>),
}
```

### 2. Garbage Collector Support
Two options:

**Option A: Global Interpreter Lock (GIL)** (Easier)
- Like Python.
- Threads exist, but only one executes bytecode at a time.
- True parallelism only for FFI calls / IO.
- Difficulty: 4/10

**Option B: Per-Thread Heap + Shared Heap** (Hard/Go-style)
- Each thread has a local heap (no locks needed).
- Shared objects move to a Global Heap (locked).
- GC allows threads to run concurrently.
- Difficulty: 9/10

### 3. Syntax Proposal (Go-style)

```rust
// Spawn a lightweight thread
spawn(fn() {
    print("Hello from thread!");
});

// Channels for communication (No shared memory hell)
let ch = Channel.new();

spawn(fn() {
    ch.send("ping");
});

let msg = ch.recv(); // "ping"
```

## Step-by-Step Implementation Plan

1.  **v3.1 (Native Threads)**: 
    - Use OS threads (1:1 model).
    - Brutal migration to `Arc<RwLock>`.
    - Simple "Stop-the-World" GC.

2.  **v3.2 (Channels)**:
    - Implement typed channels for safe communication.

3.  **v3.3 (Green Threads)**:
    - Implement user-space scheduler (like Tokio/Go) if OS threads get too heavy.
