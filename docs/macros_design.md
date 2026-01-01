# Cryo Macros Design (v2.24.0)

## Motivation
Macros allow code generation and reduction of boilerplate. They enable Domain Specific Languages (DSLs) within Cryo.

## Syntax
We will use a simple parameter substitution syntax, similar to C preprocessor but operating on AST nodes.

```javascript
macro logError(msg) {
    print("[ERROR] " + $msg);
    if (DEBUG) {
        print("Stack trace...");
    }
}

fn main() {
    logError("Something went wrong");
}
```

### Usage
`$name` indicates a parameter substitution.

## Implementation Strategy

### 1. AST Changes
- New TopLevel: `MacroDef(String, Vec<String>, Vec<Stmt>)`.
- New Stmt/Expr? 
  - Actually, we can just handle `macro` definitions during Parsing or in an Expansion pass.
  - We will treat `macro` usage as standard `Call` expressions initially.

### 2. Macro Expander Pass (`src/expander.rs`)
Runs after Parsing, before Optimization/Interpreting.

1. **Collection**: Scan AST for `macro` definitions. Store map `name -> MacroDef`. remove them from AST.
2. **Expansion**: Walk the AST.
   - If `Expr::Call(name, args)` matches a macro:
     - Clone Macro Body.
     - Recursively replace identifiers `$param` with provided argument AST `Expr`.
     - Return `Stmt::Block` (if Stmt context) or `Expr` (if Expr context).
     - **Constraint**: Macros currently will behave like Statement Blocks. Use in Expression context might be limited to single-expression macros.

### 3. Lexer/Parser
- New keyword: `macro`.
- Parse `macro name(args...) { body }`.
- Arguments in definition are just identifiers.
- In body, use `$param` (Lexer needs to assume `$` is valid identifier char or `Token::Dollar`?).
  - Let's add `$` support to `read_identifier` or standalone token.

## Roadmap
1. Update Lexer to support `macro` keyword and identifiers with `$` (or just use standard names for params like `param` and substitute matching variables?).
   - Explicit `$` is safer to distinguish macro params from locals.
   - Update Lexer to allow `$` in identifiers.
2. Update Parser to parse `MacroDef`.
3. Create `src/expander.rs`.
4. Integrate into `src/main.rs`.
