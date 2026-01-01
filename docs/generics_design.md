# Argon Generic Types Design

## Status: Phase 1 - Syntax Parsing ✅

The compiler now parses generic syntax without errors. Actual type checking
and monomorphization are not yet implemented - types are ignored at runtime.

## Overview

Generic types allow writing code that works with multiple types while maintaining type safety.

## Syntax

### Generic Functions

```argon
// Generic identity function
fn identity<T>(x: T) -> T {
    return x;
}

// Usage
let a = identity<int>(42);
let b = identity<string>("hello");
```

### Generic Structs

```argon
// Generic Pair struct
struct Pair<T, U> {
    first: T,
    second: U
}

// Usage
let p = Pair<int, string> { first: 10, second: "hello" };
print(p.first);   // 10
print(p.second);  // hello
```

## Implementation Strategy: Monomorphization

We use monomorphization - generating specialized code for each concrete type used.

### Example Transformation

**Input:**
```argon
fn identity<T>(x: T) -> T {
    return x;
}

fn main() {
    let a = identity<int>(42);
    let b = identity<string>("hi");
}
```

**Generated:**
```argon
fn identityInt(x) {
    return x;
}

fn identityString(x) {
    return x;
}

fn main() {
    let a = identityInt(42);
    let b = identityString("hi");
}
```

## AST Nodes

```
AST_GENERIC_FUNC = 130    // fn name<T>(...)
AST_TYPE_PARAMS = 131     // <T, U>
AST_TYPE_ARGS = 132       // <int, string>
```

## Parser Changes

### 1. After function name, check for `<`
```
fn parseFunction():
    expect(TOK_FN)
    name = parseIdentifier()
    
    // NEW: Check for type parameters
    type_params = []
    if current() == TOK_LT:
        advance()  // skip <
        type_params = parseTypeParamList()
        expect(TOK_GT)
    
    expect(TOK_LPAREN)
    params = parseParams()
    ...
```

### 2. At call site, check for type arguments
```
fn parseCall(name):
    // NEW: Check for type arguments
    type_args = []
    if current() == TOK_LT:
        advance()
        type_args = parseTypeArgList()
        expect(TOK_GT)
    
    expect(TOK_LPAREN)
    args = parseArgs()
    ...
```

## Code Generation Changes

### 1. Collect Generic Functions
During first pass, collect all generic function definitions.

### 2. Track Instantiations
When encountering a call like `identity<int>(42)`:
1. Look up the generic function `identity`
2. Create a specialized version `identity_int` if not exists
3. Replace call with `identityInt(42)`

### 3. Generate Specialized Functions
For each instantiation, generate a concrete function with:
- Mangled name: `{name}_{type1}_{type2}_...`
- Type parameters replaced with concrete types

## Compiler Data Structures

```argon
// Store generic function definitions
let generic_funcs = [];  // [[name, type_params, params, body], ...]

// Store instantiations to generate
let instantiations = []; // [[generic_name, [type_args], mangled_name], ...]

// Track which have been generated
let generated = [];      // [mangled_name, ...]
```

## Simplified v1 Implementation

For v1:
1. Only generic functions (not structs)
2. Explicit type arguments required (no inference)
3. No type constraints
4. Single type parameter only

## Token Changes

Add recognition of `<` and `>` in generic context.
Currently `<` is `TOK_LT` (42) and `>` is `TOK_GT` (43).

The parser needs to disambiguate:
- `a < b` (less-than comparison)
- `ident<T>` (generic instantiation)

Heuristic: After identifier, if `<` followed by identifier and then `>` or `,`, it's generic.

## Phase 1 Implementation

1. Add `isGenericContext()` helper in parser
2. Modify `parsePrimary()` to handle `ident<T>(...)`
3. Add generic storage in codegen
4. Implement monomorphization pass before IR generation
