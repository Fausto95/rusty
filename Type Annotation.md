# Type annotation

Usually types are inferred by the compiler, but they are required for functions signatures.

```rust
enum Color {
    Red,
}

fn print(msg: &str) -> bool {}

let name: String = "Hello, world!";
let num: i32 = 20000;

let color: Color = Color::Red 
```

### Generics

```rust
enum Color {
    Red,
}

let colors: Vec<Color> = vec![
    Color:Red,
    Color:Red,
    Color:Red,
];

```