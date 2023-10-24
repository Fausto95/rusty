# Modules

Modules are imported (or declared inline) using the `mod` keyword followed by the name of the file(or the inline name of the module).

```rust
//  hello.rs

pub fn say_hello() {
    println!("Hello, world!");
}


// main.rs

mod hello;

use hello::say_hello;

fn main() {
    say_hello();
}

```

## Inline modules

```rust
//  main.rs

mod hello {
    pub fn say_hello() {
        println!("Hello, world!");
    }
}

fn main() {
    hello::say_hello();
}

```