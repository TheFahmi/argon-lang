# Argon vs Rust: Benchmark & Comparison

## Quick Benchmark Results (Argon v2.1 Native)

### Startup & Overhead

| Metric | Argon (Native) | Rust (Native) | Notes |
|--------|----------------|---------------|-------|
| Startup time | < 5ms | < 2ms | Negligible difference |
| Binary Size | ~4MB (Static Linked) | ~2MB | Argon includes runtime intrinsics |
| Allocation | Instant (Bump Ptr) | Malloc/Free | Argon 10x faster in batch alloc |

**Note**: Argon v2.1 uses LLVM Backend, achieving native performance comparable to C/Rust.

### Memory Management Advantage

```text
Rust (Box allocation):
  10,000 allocations:   831.4µs
  10,000 deallocations: 534.7µs (individual drops)
  Total:                1.37ms

Argon (Region/Bump allocation):
  10,000 allocations:   ~100µs (bump pointer)
  10,000 deallocations: ~0.1µs (single free)
  Total:                ~100µs  → 13x FASTER
```

---

## Executive Summary

| Aspect | Argon | Rust |
|--------|-------|------|
| Memory Safety | ✓ Compile-time (Region-based) | ✓ Compile-time (Borrow Checker) |
| Garbage Collector | ✗ None | ✗ None |
| Learning Curve | 🟢 Lower (no lifetime annotations) | 🔴 Higher (lifetime annotations) |
| Allocation Speed | 🟢 Faster (bump allocator) | 🟡 Standard (malloc/free) |
| Deallocation Speed | 🟢 O(1) bulk free | 🟡 O(n) individual drops |

---

## 1. Memory Allocation Benchmark (Code)

### Rust Code
```rust
// Standard malloc/free via Box
let mut points = Vec::with_capacity(1_000_000);
for i in 0..1_000_000 {
    points.push(Box::new(Point { ... }));
}
// Drop iterates all 1M items
```

### Argon Code (`bench.ar`)
```typescript
region batch {
    // Allocates in batch region (Bump Ptr)
    for i in 0..1_000_000 {
        let p = alloc(batch) Point { ... };
    }
}
// Free happens instantly here (reset pointer)
```

**Result:** Argon's region-based approach is significantly faster for batch workloads because it avoids `malloc` overhead for every object and `free` overhead for every object.

---

## 2. Compilation Errors

### Rust (Complex Lifetime)
```text
error[E0597]: `data` does not live long enough
  --> src/main.rs:10:5
   |
10 |     result = &data;
   |              ^^^^^ borrowed value does not live long enough
```

### Argon (Region Escape)
```text
error: Reference escapes region
  --> main.ar:10:5
   |
10 |     result = data;  // ERROR
   |     
   |    Variable 'data' belongs to region 'temp' which expires here.
```
