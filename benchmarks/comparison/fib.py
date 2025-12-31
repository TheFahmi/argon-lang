import time

def fib(n):
    if n < 2: return n
    return fib(n-1) + fib(n-2)

print("Python: Starting Fib(25)...")
start = time.time() * 1000
res = fib(25)
end = time.time() * 1000
print(f"Python: Result = {res}")
print(f"Python: Time = {end - start:.2f}ms")
