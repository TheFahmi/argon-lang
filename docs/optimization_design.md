# Cryo Optimization Design (v2.22.0)

## Motivation
Cryo v2.21.0 interpreter is fully functional with GC, but can be slow and stack-heavy due to:
1. Recalculating constants at runtime (`1 + 2` is computed every time).
2. Deep recursion causing Stack Overflow (Rust stack is limited).

We aim to implement optimizations to address these.

## 1. Constant Folding (Compile-Time Evaluation)

We will implement a `optimize_ast` pass that transforms the AST before execution.

**Transformation Rules:**
- `BinOp(Int(a), "+", Int(b))` -> `Int(a + b)`
- `BinOp(Bool(a), "&&", Bool(b))` -> `Bool(a && b)`
- `If(Bool(true), then, else)` -> `Block(then)`
- `If(Bool(false), then, else)` -> `Block(else)` (or None)

**Example:**
```javascript
let x = 2 * 3 + 4; // Becomes: let x = 10;
if (DEBUG) { ... } // If DEBUG is const false, block is removed.
```

## 2. Tail Call Optimization (TCO)

Tail recursion (`return fn(...)`) currently adds a Stack Frame in Rust. Deep recursion crashes the interpreter.

**Strategy: Trampolining**
Instead of calling `execute_function` recursively immediately, we return a special `TailCall(Function, Args)` value called a "Thunk". 
The top-level loop works like this:

```rust
loop {
    match result {
        Value::TailCall(func, args) => {
             // Re-use current stack frame logic, just swap func/args
             current_func = func;
             current_args = args;
             continue;
        },
        val => return val
    }
}
```

**Instruction:**
- Modify `Interpreter::call_function` to detect Tail Position.
- In `Stmt::Return(Expr::Call(...))`, allow emitting a TailCall control flow.
- Modify `Interpreter::execute_function` to loop on Tail Calls.

## Implementation Steps
1. Create `optimizer.rs` module for AST optimization.
2. Integrate `optimize()` into `Interpreter::run`.
3. Implement TCO in `interpreter.rs`.

## Benchmarks
- `fib(30)` or deeply recursive function to test TCO.
