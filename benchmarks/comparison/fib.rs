use std::time::Instant;

fn fib(n: i64) -> i64 {
    if n < 2 { return n; }
    fib(n - 1) + fib(n - 2)
}

fn main() {
    println!("Rust: Starting Fib(35)...");
    let start = Instant::now();
    let res = fib(35);
    let duration = start.elapsed();
    println!("Rust: Result = {}", res);
    println!("Rust: Time = {}ms", duration.as_millis());
}
