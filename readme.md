# Boudica Scripting Language

An embeddable, modern, ergonomic, and statically typed scripting language.

## Some Quick Examples

Hello world:

```
include {println} from system;

let main = ||{
    println("Hello World!");
}
```

Adding numbers

```
include {println} from system;

let add = |a:i32, b:i32|:i32 {
    a + b // Like rust, the last expression in a function body is returned automatically.
}

let main = ||{
    println(add(5, 7));
}
```

Recursive Fibonacci
```
include {println} from system;

/*
* Doc strings! Document your code!
* This is a function that calculates Fibonacci numbers.
*/
let fib = |n: i32|:i32 {
    if n < 0 {0}
    else if n < 2 {1}
    else {fib(n-1)+fib(n-2)}
}

let main = ||{
    for i in 0..10{ // use of range operator. The implicit type of i is u64
        println(f"fib of {i} is {fib(i)}"); // F strings for formatting, like Python
    }
}
```

Structs

```
include {println} from system;
include {sqrt} from math;

struct Vec2{
    x: i32,
    y: i32,
}

// Attach a static method to Vec2
let Vec2::new = |x: i32, y: i32|: Self {
    Self{x, y}
}
// Attach a magic instance method
let Vec2:__add__ = |self: Vec2, v: Vec2|{ 
    // Create or overload infix/unary operators.
    // It's a powerful tool, don't abuse it.
    Vec2::new(self.x + v.x, self.y + v.y) // Create a new vector2, leaving the two originals untouched.
}
let Vec2:normalize = |self: Vec2|{
    // Normalize a vector in place
    let len = sqrt(self.x * self.x + self.y*self.y);
    self.x /= len;
    self.y /= len;
    self
}

let Vec2:__str__ = |self: Vec2| {
    f"x: {self.x}, y: {self.y}"
}

let main = ||{
    let a = Vec2::new(1, 1);
    let b = Vec2::new(3, 1);
    let c = a + b;
    println(c:normalize());
}

```

## Non-Features

- Exceptions
- Null

## Eventual Features

- Generics
- Language Server
- Doc comments

## Random Notes

Non ascii characters are only allowed in comments and strings.
Accessing characters by index in a string is supported. For all ascii strings (the common case) this can be performed in constant time. For strings with non ascii characters this takes non constant time. This is handled internally by having ascii strings and non ascii strings be different types.

Strings are valid utf8. Path strings and utf16 stuff is a later me problem.

Creating new operators at compile time is a powerful and complex feature, and maybe isn't a good idea. Still, it's interesting to at least explore. Worst case scenario only overriding existing operators is allowed. That's probably how it'll work at least in the short term.

Destructuring structs and arrays is allowed, like JS or TS. Tuples too, as per Python.

Unlike Rust, variables are mutable by default. There should probably be a const keyword.

Exports and imports work similarly to TS. Don't fix what ain't broke.

Modules can be written in-tree, or provided by the host via Boudica's ffi.
