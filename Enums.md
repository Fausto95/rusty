# Enums

Enums are a piece of data that can be one of multiple possibilities - Each possibility is called a `variant`.

Provides information about our program to the compiler.

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn main() {
    let direction = Direction::Up;
    
    match direction {
        Direction::Up => println!("Going up!"),
        Direction::Down => println!("Going down!"),
        Direction::Left => println!("Going left!"),
        Direction::Right => println!("Going right!")
    }
}
```