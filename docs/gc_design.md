# Argon Garbage Collection & Memory Model Design (v2.21.0)

## Motivation
Currently, Argon (v2.20.0) reference interpreter uses **Copy Semantics** for all types.
- `let a = [1, 2]; let b = a;` creates a deep copy. Modifying `b` does not affect `a`.
- Passing arrays/structs to functions copies them.

This has two downsides:
1. **Performance**: Copying large arrays/structs is O(N).
2. **Behavior**: Systems programming and scripting often require shared state.

"Garbage Collection" in this context implies moving to **Reference Semantics** where objects are allocated on a heap and managed automatically.

## Proposed Changes

### 1. Value Types vs Reference Types
We will split types into two categories:

**Value Types (Copy Semantics):**
- `Null`
- `Bool`
- `Int` (i64)
- `Float` (f64) - *Proposed addition*

**Reference Types (Pointer Semantics):**
- `String` (Immutable reference or Copy? Usually Copy in primitive-like languages, Ref in Java. We'll stick to **Copy** for Strings for now or **Ref**? String mutation `s[0] = 'a'`? Argon strings seem immutable in stdlib `to_upper` returns new string. Let's keep Strings immutable value-like).
- `Array` (Mutable Reference)
- `Struct` (Mutable Reference)
- `Function` (Reference)

### 2. Implementation Strategy (Rust Interpreter)

We will modify the `Value` enum to use `Rc<RefCell<T>>` for reference types. This implements **Reference Counting (RC)** GC.

```rust
use std::rc::Rc;
use std::cell::RefCell;

pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    // Strings are immutable, so Rc is enough (no RefCell needed unless we want in-place mutation)
    // But for simplicity of "value semantics", String(String) is fine if immutable.
    String(String), 
    
    // Reference Types
    Array(Rc<RefCell<Vec<Value>>>),
    Struct(Rc<RefCell<HashMap<String, Value>>>),
    
    // Closures might need env capture later
    Function(String, Vec<Param>, Option<Vec<Stmt>>), 
}
```

### 3. Impact on Code

**Assignments:**
```javascript
let list = [1, 2, 3];
let alias = list;      // alias points to SAME memory
alias.push(4);
print(list);           // Output: [1, 2, 3, 4]
```

**Function Calls:**
```javascript
fn update(arr) {
    arr.push(99);
}
let data = [];
update(data);
print(data); // Output: [99] (Previously would be [])
```

### 4. Cycle Handling
With simple `Rc`, reference cycles (A -> B -> A) will cause memory leaks.
- **Phase 1**: Ignore leaks (standard approach for simple interpreters).
- **Phase 2**: Implement cycle collector or `Weak` references.

## Roadmap
1. Modify `Value` enum in `src/interpreter.rs`.
2. Update `PartialEq`, `Clone`, and `Debug` implementations.
3. Update `eval_binop` and built-ins to handle `RefCell`.
4. Test with `examples/gc_example.ar`.
