# Cryo Traits Design (v2.20.0)

## Overview

Traits define shared behavior (interfaces) that types can implement.

## Syntax

### Defining Traits

```cryo
trait Printable {
    fn toString(self) -> string;
}

trait Comparable {
    fn compare(self, other: Self) -> i32;
    fn equals(self, other: Self) -> bool;
}

trait Iterator<T> {
    fn next(self) -> Option<T>;
    fn hasNext(self) -> bool;
}
```

### Implementing Traits

```cryo
struct Point {
    x: i32,
    y: i32
}

impl Printable for Point {
    fn toString(self) -> string {
        return "(" + self.x + ", " + self.y + ")";
    }
}

impl Comparable for Point {
    fn compare(self, other: Point) -> i32 {
        let d1 = self.x * self.x + self.y * self.y;
        let d2 = other.x * other.x + other.y * other.y;
        return d1 - d2;
    }
    
    fn equals(self, other: Point) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}
```

### Trait Bounds (Generic Constraints)

```cryo
// Function that requires Printable
fn print_it<T: Printable>(item: T) {
    print(item.toString());
}

// Multiple bounds
fn compare_and_print<T: Comparable + Printable>(a: T, b: T) {
    print("Comparing: " + a.toString() + " vs " + b.toString());
    let result = a.compare(b);
    if (result < 0) {
        print("First is smaller");
    } else if (result > 0) {
        print("First is larger");
    } else {
        print("They are equal");
    }
}
```

### Default Implementations

```cryo
trait Display {
    fn format(self) -> string;
    
    // Default implementation
    fn display(self) {
        print(self.format());
    }
}

impl Display for Point {
    fn format(self) -> string {
        return "Point(" + self.x + ", " + self.y + ")";
    }
    // display() uses default implementation
}
```

## Implementation

### Token Changes

```cryo
let TOK_TRAIT = 92;       // trait keyword
let TOK_IMPL = 93;        // impl keyword
let TOK_FOR = 94;         // for keyword
let TOK_SELF_TYPE = 95;   // Self type
```

### AST Changes

```cryo
let AST_TRAIT_DEF = 90;   // trait Name { ... }
let AST_IMPL_BLOCK = 91;  // impl Trait for Type { ... }
let AST_TRAIT_BOUND = 92; // T: Trait
let AST_SELF_TYPE = 93;   // Self keyword
```

### Type System

```
TraitDef {
    name: string,
    methods: [MethodSig],
    default_impls: [Function]
}

ImplBlock {
    trait_name: string,
    type_name: string,
    methods: [Function]
}

TraitBound {
    type_param: string,
    traits: [string]
}
```

### Code Generation (Monomorphization)

Traits are implemented via monomorphization (like generics):

```cryo
// Source
fn print_it<T: Printable>(item: T) { ... }
printIt(point);   // Point
printIt(vec);     // Vec2

// Generated
fn print_it_Point(item: Point) { ... }
fn print_it_Vec2(item: Vec2) { ... }
```

## Built-in Traits

### Clone
```cryo
trait Clone {
    fn clone(self) -> Self;
}
```

### Default
```cryo
trait Default {
    fn default() -> Self;
}
```

### Debug
```cryo
trait Debug {
    fn debug(self) -> string;
}
```

### Eq / Ord
```cryo
trait Eq {
    fn eq(self, other: Self) -> bool;
    fn ne(self, other: Self) -> bool {
        return !self.eq(other);
    }
}

trait Ord: Eq {
    fn cmp(self, other: Self) -> i32;
    fn lt(self, other: Self) -> bool { return self.cmp(other) < 0; }
    fn gt(self, other: Self) -> bool { return self.cmp(other) > 0; }
    fn le(self, other: Self) -> bool { return self.cmp(other) <= 0; }
    fn ge(self, other: Self) -> bool { return self.cmp(other) >= 0; }
}
```

### Iterator
```cryo
trait Iterator<T> {
    fn next(self) -> Option<T>;
    
    fn map<U>(self, f: fn(T) -> U) -> MapIterator<T, U>;
    fn filter(self, pred: fn(T) -> bool) -> FilterIterator<T>;
    fn collect(self) -> [T];
}
```

## Phase Implementation

1. **Phase 1**: trait definition
2. **Phase 2**: impl Trait for Type
3. **Phase 3**: Trait bounds <T: Trait>
4. **Phase 4**: Default implementations
5. **Phase 5**: Built-in traits
6. **Phase 6**: Trait inheritance (trait A: B)

## Example Program

```cryo
trait Animal {
    fn speak(self) -> string;
    fn name(self) -> string;
}

struct Dog { name: string }
struct Cat { name: string }

impl Animal for Dog {
    fn speak(self) -> string { return "Woof!"; }
    fn name(self) -> string { return self.name; }
}

impl Animal for Cat {
    fn speak(self) -> string { return "Meow!"; }
    fn name(self) -> string { return self.name; }
}

fn introduce<T: Animal>(animal: T) {
    print(animal.name() + " says: " + animal.speak());
}

fn main() {
    let dog = Dog { name: "Buddy" };
    let cat = Cat { name: "Whiskers" };
    
    introduce(dog);  // "Buddy says: Woof!"
    introduce(cat);  // "Whiskers says: Meow!"
}
```
