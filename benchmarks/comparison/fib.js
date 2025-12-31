function fib(n) {
    if (n < 2) return n;
    return fib(n - 1) + fib(n - 2);
}

console.log("NodeJS: Starting Fib(25)...");
const start = Date.now();
const res = fib(25);
const end = Date.now();
console.log("NodeJS: Result = " + res);
console.log("NodeJS: Time = " + (end - start) + "ms");
