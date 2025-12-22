# Argon Language: Initial Design Document

**Version:** 0.1.0-draft
**Target Audience:** Systems Programmers, Compiler Engineers
**Code Name:** Argon

---

## 1. The Safety Mechanism: "Scoped Regions & Linear Capabilities"

The biggest hurdle in Rust is the cognitive load of lifetime annotations (`'a`) and the Borrow Checker's granular analysis. Argon proposes a different model that achieves memory safety without GC, focusing on **First-Class Regions** combined with **Linear Capabilities**.

### The Core Concept: Allocation Regions (Arenas) as First-Class Citizens
Instead of tracking the lifetime of every single object individually, Argon groups object lifetimes into *Regions*.

1.  **Region-Based Memory Management (RBMM):**
    *   All heap allocations must happen within a `Region`.
    *   A Region is a memory arena (bump allocator). Deallocation happens *en masse* when the Region goes out of scope.
    *   This eliminates memory fragmentation and creates extremely fast allocation/deallocation cycles (zero-cost abstraction).
    *   **Safety Rule:** A reference to an object in Region A cannot outlive Region A.

2.  **Linear Capabilities (Ownership 2.0):**
    *   Objects that own resources (sockets, file handles, or unique pointers to other regions) are *Linear*.
    *   They must be used exactly once (moved) or explicitly destroyed.
    *   There is no "implicit drop" for complex resources, forcing the developer to handle cleanup paths (eliminating resource leaks).

3.  **Why it kills Rust's Complexity:**
    *   No explicit lifetime parameters `<'a>` on functions in 90% of cases.
    *   The compiler infers lifetimes based on Region scoping.
    *   Cyclic data structures (a pain in Rust) are trivial inside a single Region (references within the same region can cycle safely).

### Comparison
*   **Rust:** `&'a val`. Granular, precise, but high cognitive load.
*   **Argon:** `val @ Region`. Coarser (bulk freeing), but significantly simpler mental model and faster runtime performance for batch workloads (compilers, game engines, servers).

---

## 2. Syntax Philosophy & Examples

Argon aims for a "Refined C-Family" aesthetic. usage is read-heavy, so explicit keywords are preferred over symbols.

### Philosophy
*   **Explicit Mutability:** Everything is immutable by default.
*   **Expression-Based:** Like Rust/Ruby, blocks return values.
*   **No Header Files:** Module-based import system.

### Examples

#### Variable Declaration
```argon
// Immutable by default
let pi: f64 = 3.14159;

// Mutable variable
mut counter: i32 = 0;
counter += 1;

// Type Inference
let message = "Hello, World"; // Inferred as StringSlice
```

#### Functions & Return Values
```argon
// explicit 'fn' keyword
// return type follows '->'
fn calculate_area(radius: f64) -> f64 {
    let area = 3.14 * radius * radius;
    return area; // Explicit return preferred, but implicit last-expression works too
}

// Result type for error handling (algebraic implementation)
fn divide(a: i32, b: i32) -> Result<i32, Error> {
    if (b == 0) {
        return Error("Division by zero");
    }
    return Ok(a / b);
}
```

#### Structs & Methods (Data-Oriented)
Argon separates Data (Structs) from Behavior (Traits/Impls).

```argon
// A pure data structure (POD)
struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

// Implementation block
impl Vector3 {
    // Static constructor ("new" is just a convention)
    fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    // Method taking a reference (borrowing)
    fn mag_sq(self: &Vector3) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    
    // Method modifying self
    fn normalize(mut self: &Vector3) {
        let m = sqrt(self.mag_sq());
        self.x /= m;
        self.y /= m;
        self.z /= m;
    }
}
```

#### Safety in Action: The `region` block
This demonstrates the Region-based safety.

```argon
fn process_requests() {
    // 1. Permanent/Global Region exists by default (heap)
    let global_config = new(Global) Config();

    // 2. Transitive Region (Scope-based Arena)
    region request_scope {
        // 'alloc' keyword specifies the region
        let user_data = alloc(request_scope) UserData::fetch(101);
        
        // References are valid here
        validate(user_data);
        
        // COMPILE ERROR: Cannot move 'user_data' out of 'request_scope'
        // global_config.last_user = user_data; 
    } 
    // 'request_scope' acts like an Arena. 
    // All memory allocated via 'alloc(request_scope)' is freed here instantly.
    // No individual destructors run for POD types (speed!).
}
```

---

## 3. The Compiler Architecture Roadmap

We will write the bootstrap compiler in **Modern C++ (C++20)**. This gives us direct, zero-overhead access to the LLVM C++ API (which is written in C++) and establishes Argon as a standalone system without depending on its rival's toolchain.

### Phase 0: Setup
*   **Language:** C++20 (GCC 11+ or Clang 14+).
*   **Build System:** CMake (Standard for LLVM-based projects).
*   **Dependencies:** `LLVM` (Core, Support, IR, Analysis, Target).

### Phase 1: Frontend (The Parser)
**Goal:** Source Code -> Abstract Syntax Tree (AST).
1.  **Lexer (Tokenization):** A hand-written state-machine lexer in C++.
    *   Input: `std::string_view` (source file).
    *   Output: `std::vector<Token>`.
    *   Why hand-written? Maximum speed and easiest to generate "did you mean?" error suggestions.
2.  **Parser (Syntax Analysis):** **Recursive Descent Parser**.
    *   Structure: A class `Parser` consuming the token stream.
    *   Output: `std::unique_ptr<AST::Node>` (Smart pointers for AST ownership).
3.  **Module System:** A simple filesystem crawler to resolve `import` statements.

### Phase 2: Semantic Analysis (The Brain)
**Goal:** AST -> Typed High-Level IR (HIR) + Safety Guarantees.
*   **Symbol Table:** `std::unordered_map<std::string, SymbolInfo>` handling scopes.
*   **Type Checker:** Implement a unification algorithm (Hindley-Milner) tailored for C-like syntax.
*   **The "Region Verifier" (The Police):**
    *   Traverse the AST *after* type checking.
    *   Simulate the "Region Stack".
    *   Ensure no pointer in `Region<Inner>` is assigned to a variable in `Region<Outer>` without a linear move.

### Phase 3: Code Generation (The Backend)
**Goal:** HIR -> LLVM IR -> Native Machine Code.
1.  **LLVM Context:** Initialize `llvm::LLVMContext` and `llvm::Module`.
2.  **IR Builder:** Use `llvm::IRBuilder<>` to generate instructions.
    *   Data structures (structs) map to `llvm::StructType`.
    *   Regions map to calls to intrinsic memory functions (e.g., `@argon_alloc(region_ptr, size)`).
3.  **Optimization:** Use the new `llvm::PassManagerBuilder` to run default O3 pipelines.
4.  **Driver:** A simple `main.cpp` that invokes `clang` or `lld` to link the object files.

### Implementation Status (✓ = Done)

| Component | Status | File |
|-----------|--------|------|
| Project Setup | ✓ | `CMakeLists.txt` |
| Token Definitions | ✓ | `include/argon/Token.h` |
| Lexer | ✓ | `include/argon/Lexer.h`, `src/Lexer.cpp` |
| AST Nodes | ✓ | `include/argon/AST.h` |
| Parser | ✓ | `include/argon/Parser.h`, `src/Parser.cpp` |
| Type System | ✓ | `include/argon/Type.h` |
| Type Checker | ✓ | `include/argon/TypeChecker.h`, `src/TypeChecker.cpp` |
| Region Verifier | ✓ | `include/argon/RegionVerifier.h`, `src/RegionVerifier.cpp` |
| Code Generator | ✓ | `include/argon/CodeGen.h`, `src/CodeGen.cpp` |
| Main Driver | ✓ | `src/main.cpp` |

### Next Steps (TODO)
1.  ~~Initialize project: `mkdir argc && cd argc`.~~ ✓
2.  ~~Create `CMakeLists.txt` finding LLVM package.~~ ✓
3.  ~~Write `include/argon/Token.h`.~~ ✓
4.  Implement struct/impl parsing.
5.  Add function parameters and return type parsing.
6.  Implement `alloc(region)` expression parsing.
7.  Add match/pattern matching syntax.
8.  Implement trait system.
9.  Add standard library runtime (argon_alloc, argon_free_region).
10. Self-hosting: rewrite compiler in Argon itself.

