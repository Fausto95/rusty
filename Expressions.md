# Expressions

Most things in Rust are expression-based, meaning that they're evaluated and return some value

`If` and `match` expressions can be nested.

```rust
fn main() {
    let age = 27;
    let is_adult = if age >= 18 { true } else { false };
    let is_adult = age >= 18;

    let is_adult = match age {
        0 => false,
        1..=12 => false,
        13..=19 => true,
        _ => true,
    };

    println!("is adult: {}", is_adult);
}
```