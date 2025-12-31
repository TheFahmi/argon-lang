#include <iostream>
#include <chrono>

long long fib(long long n) {
    if (n < 2) return n;
    return fib(n - 1) + fib(n - 2);
}

int main() {
    std::cout << "C++: Starting Fib(35)..." << std::endl;
    auto start = std::chrono::high_resolution_clock::now();
    long long res = fib(35);
    auto end = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);
    
    std::cout << "C++: Result = " << res << std::endl;
    std::cout << "C++: Time = " << duration.count() << "ms" << std::endl;
    return 0;
}
