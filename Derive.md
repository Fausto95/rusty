# Derive

Without `derive`, if we needed to print all of the variants of an enum, we would need to through each one of them.
Derive is a special macro, that is applied to `enum` & `struct`;


### Debug

```rust
#[derive(Debug)] // Without this the program would throw an error
enum Color {
    Red,
}

fn main() {
    println!("{:?}", Color::Red);
}

```

### Clone & Copy

Used to clone & copy ownsership