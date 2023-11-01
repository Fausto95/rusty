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

Enums variants can contain data, which can also be another enum.


```rust
enum Direction {
    Left(i64),
    Move(i8, i32)
}

// Ignoring the arg
match direction {
    Direction::Left(_) => println!("Dir left"),
}

// Matching specific arg
match direction {
    Direction::Left(20) => println!("Dir left 20"),
}

// Given a name to the arg so we can reference later on
match direction {
    Direction::Left(amount) => println!("Dir left {:?}", amount),
    _ => (),
}
```