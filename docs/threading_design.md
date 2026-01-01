# Cryo Threading & Concurrency

**Version:** v3.1.0  
**Status:** ✅ Implemented

## Overview

Cryo provides **true OS-level parallelism** through its threading module. This enables real parallel execution using native operating system threads, not simulated concurrency.

## Features

- **OS Threads**: Real parallel execution using `std::thread`
- **Channels**: Thread-safe message passing (mpsc)
- **Non-blocking Operations**: Try-receive and timeout support
- **Worker API**: Simple spawn/join semantics

## Built-in Functions

### Thread Management

| Function | Description |
|----------|-------------|
| `threadSpawn(value, operation)` | Spawn a new OS thread with a computation |
| `threadJoin(worker_id)` | Wait for thread completion and get result |
| `threadIsDone(worker_id)` | Check if thread has finished |
| `threadActiveCount()` | Get number of active threads |

### Channel Communication

| Function | Description |
|----------|-------------|
| `channelNew()` | Create a new unbuffered channel |
| `channelSend(ch, value)` | Send value to channel (returns bool) |
| `channelRecv(ch)` | Receive from channel (blocking) |
| `channelTryRecv(ch)` | Receive without blocking (returns null if empty) |
| `channelRecvTimeout(ch, ms)` | Receive with timeout in milliseconds |
| `channelClose(ch)` | Close the channel |

## Supported Operations

When spawning a thread with `threadSpawn(value, operation)`, the following operations are available:

| Operation | Description | Example |
|-----------|-------------|---------|
| `"fib"` | Compute Fibonacci number | `threadSpawn(30, "fib")` |
| `"factorial"` | Compute factorial | `threadSpawn(10, "factorial")` |
| `"double"` | Multiply by 2 | `threadSpawn(21, "double")` |
| `"square"` | Square the value | `threadSpawn(5, "square")` |
| `"sleep"` | Sleep for N milliseconds | `threadSpawn(100, "sleep")` |

## Examples

### Parallel Fibonacci

```cryo
fn main() {
    // Spawn 4 parallel fibonacci computations
    let t1 = threadSpawn(30, "fib");
    let t2 = threadSpawn(31, "fib");
    let t3 = threadSpawn(32, "fib");
    let t4 = threadSpawn(33, "fib");
    
    // Wait for all results
    let r1 = threadJoin(t1);
    let r2 = threadJoin(t2);
    let r3 = threadJoin(t3);
    let r4 = threadJoin(t4);
    
    print("fib(30) = " + r1);  // 832040
    print("fib(31) = " + r2);  // 1346269
    print("fib(32) = " + r3);  // 2178309
    print("fib(33) = " + r4);  // 3524578
}
```

### Channel Communication

```cryo
fn main() {
    // Create channel
    let ch = channelNew();
    
    // Send messages
    channelSend(ch, 42);
    channelSend(ch, 100);
    channelSend(ch, 999);
    
    // Receive messages (FIFO order)
    let v1 = channelRecv(ch);  // 42
    let v2 = channelRecv(ch);  // 100
    let v3 = channelRecv(ch);  // 999
    
    print("Received: " + v1 + ", " + v2 + ", " + v3);
}
```

### Non-blocking Receive

```cryo
fn main() {
    let ch = channelNew();
    
    // Try to receive from empty channel
    let result = channelTryRecv(ch);
    if (result == null) {
        print("Channel is empty");
    }
    
    // Send something
    channelSend(ch, 777);
    
    // Now try_recv will succeed
    let value = channelTryRecv(ch);
    print("Got: " + value);  // 777
}
```

### Timeout Receive

```cryo
fn main() {
    let ch = channelNew();
    
    // Wait up to 100ms for a message
    let result = channelRecvTimeout(ch, 100);
    
    if (result == null) {
        print("Timeout - no message received");
    } else {
        print("Got: " + result);
    }
}
```

### Check Thread Status

```cryo
fn main() {
    let worker = threadSpawn(500, "sleep");
    
    // Check if done immediately
    if (!threadIsDone(worker)) {
        print("Worker is still running...");
    }
    
    // Wait for completion
    let result = threadJoin(worker);
    print("Worker finished");
}
```

## Implementation Details

The threading module is implemented in Rust using:

- `std::thread` for OS thread management
- `std::sync::mpsc` for channel communication
- `Arc<Mutex<>>` for thread-safe shared state

### Architecture

```
┌─────────────────────────────────────────┐
│           Cryo Interpreter             │
├─────────────────────────────────────────┤
│          ThreadManager                  │
│  ┌─────────────┐  ┌─────────────────┐  │
│  │   Workers   │  │    Channels     │  │
│  │ HashMap<id> │  │ Sender/Receiver │  │
│  └─────────────┘  └─────────────────┘  │
├─────────────────────────────────────────┤
│           OS Threads (std::thread)      │
└─────────────────────────────────────────┘
```

### Thread Safety

- `ThreadValue` is a serializable enum that can be safely passed between threads
- Channels use Rust's `mpsc` (multi-producer, single-consumer)
- The `Receiver` is wrapped in `Arc<Mutex<>>` for safe sharing

## Limitations

1. **Custom Functions**: Currently only predefined operations (fib, factorial, etc.) can be spawned. Custom Cryo functions cannot be executed in parallel yet.

2. **Structs**: Struct values cannot be sent through channels (they serialize to Null).

3. **Functions**: Function values cannot be sent through channels.

## Future Enhancements

- [ ] Spawn arbitrary Cryo functions in threads
- [ ] Work-stealing thread pool
- [ ] Async/await integration
- [ ] Parallel iterators

## See Also

- [Channel Module](../stdlib/channel.ar) - High-level channel patterns
- [Worker Module](../stdlib/worker.ar) - Worker pool abstractions
