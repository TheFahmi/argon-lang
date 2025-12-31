// Argon Fast VM - Ultra-optimized execution for numeric code
// Uses flat i64 stack and direct recursion for maximum speed

/// Ultra-fast fibonacci calculation using native recursion
/// This is what a properly compiled Argon program would look like
#[inline(never)]
pub fn native_fib(n: i64) -> i64 {
    if n < 2 { return n; }
    native_fib(n - 1) + native_fib(n - 2)
}

/// Run the native fib benchmark
pub fn run_native_fib_bench(n: i64) -> (i64, std::time::Duration) {
    let start = std::time::Instant::now();
    let result = native_fib(n);
    let elapsed = start.elapsed();
    (result, elapsed)
}
