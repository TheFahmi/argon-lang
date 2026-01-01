# Argon Defer Statement Design (v2.23.0)

## Motivation
Resource management (closing files, freeing memory) is critical, especially with the new FFI system.
Full RAII (Destructors) is difficult to implement in the current Interpreter architecture because Rust's `Drop` trait cannot access the Interpreter state to execute Argon code.

**Solution**: Go-style `defer` statement.

## Syntax
```javascript
fn processFile(path: string) {
    let f = open(path);
    defer close(f); // Schedules execution for scope exit
    
    // ... work with f ...
    
    if (error) {
        return; // close(f) executes here too
    }
} // close(f) executes here
```

## Implementation Strategy

### 1. AST Changes
Add `Stmt::Defer(Box<Stmt>)` (or `Expr`?).
Usually `defer` takes a function call expression.
`Stmt::Defer(Expr)`.

### 2. Interpreter Changes
Modify `Interpreter` to track deferred actions in the current scope.
`stack: Vec<Scope>` where `Scope` contains variables AND a list of deferred actions.

```rust
struct Scope {
    variables: HashMap<String, Value>,
    deferred: Vec<Stmt>, // or Expr
}
```

**Execution Flow:**
- When `defer expr` is encountered: Push `expr` to current scope's deferred list.
- When `popScope()` is called (end of block, return, break):
    - Iterate `deferred` list in **reverse order** (LIFO).
    - Execute each stmt/expr.

### 3. Edge Cases
- **Return**: When `return` happens, we must execute deferred statements of *all* scopes being popped.
- **Break/Continue**: Same, execute deferred statements of scopes being exited.
- **Error/Panic**: Should also invoke deferred (if we had panic recovery).

## Roadmap
1. Update `Lexer` & `Parser` for `defer` keyword.
2. Update `Interpreter` struct to manage deferred stack.
3. Update `exec_stmt` to handle `Defer` and scope exits.
