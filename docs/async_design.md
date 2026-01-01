# Cryo Async/Await Design (v2.18.0)

## Overview

Add asynchronous programming support to Cryo with:
- `async fn` - Define asynchronous functions
- `await` - Wait for async operations
- `Future<T>` - Type representing pending computation
- Event loop for concurrent execution

## Syntax

### Async Function Declaration
```cryo
async fn fetchData(url: string) {
    let response = await httpGet(url);
    return response;
}
```

### Await Expression
```cryo
fn main() {
    let data = await fetchData("http://example.com");
    print(data);
}
```

### Multiple Concurrent Tasks
```cryo
async fn main() {
    // Start tasks concurrently
    let task1 = spawn fetchData("url1");
    let task2 = spawn fetchData("url2");
    
    // Wait for all
    let r1 = await task1;
    let r2 = await task2;
}
```

## Implementation Strategy

### Phase 1: Token & Parser Support
1. Add `TOK_ASYNC` and `TOK_AWAIT` tokens
2. Parse `async fn` declarations
3. Parse `await <expr>` expressions
4. Store async flag in AST_FUNC nodes

### Phase 2: Runtime Support
1. Add `Future` struct in runtime
2. Implement simple event loop
3. Add `cryoSpawn()` - Create new async task
4. Add `cryoAwait()` - Wait for task completion

### Phase 3: Code Generation
1. Transform async functions to state machines
2. Generate continuation-passing style (CPS) code
3. Emit Future creation and resolution

## AST Changes

### AST_ASYNC_FUNC (new node type = 140)
```
[140, name, params, body, is_async]
```

### AST_AWAIT (new node type = 141)
```
[141, expr]
```

## Runtime Functions

```c
// Create a new async task
i64 cryoAsyncSpawn(i64 fn_ptr);

// Wait for task completion
i64 cryoAsyncAwait(i64 task_id);

// Check if task is ready
i64 cryoAsyncPoll(i64 task_id);

// Simple event loop tick
void cryoAsyncTick();
```

## Simple Implementation (v1)

For v1, we'll use a simple thread-based approach:
- `async fn` creates a new thread
- `await` blocks until thread completes
- No complex state machine transformation needed

This is simpler but still provides useful async semantics.

## Example

```cryo
import "stdlib/async"

async fn slowOperation() {
    sleep(1000);  // 1 second
    return 42;
}

async fn main() {
    print("Starting...");
    let result = await slowOperation();
    print("Result: " + result);
}
```

## Files to Modify

1. `self-host/compiler.cryo` - Add async/await tokens and parsing
2. `self-host/runtime.rs` - Add async runtime functions
3. `stdlib/async.cryo` - Async utilities module
4. `examples/async_example.cryo` - Demo

## Token IDs
- `TOK_ASYNC = 80`
- `TOK_AWAIT = 81`

## AST Node IDs
- `AST_ASYNC_FUNC = 140`
- `AST_AWAIT = 141`
