# Variables

```rust
let name = "Faustino";
let initialLetter = 'F';

let age = 27;
let half = 0.5;

let mut last_name = "Kial";
last_name = "Liak";

let isAngolan = true;
let copy = half;
```

In Rust, variables are declared using the `let` keyword and they are immutable by default.

We can do shadowing by redefining the same variable.

```rust
let file = read_file(input);
let file = process_file(file);
let file = trim_file(file);
```

To make a variable mutable, we use the keyword `mut`.
