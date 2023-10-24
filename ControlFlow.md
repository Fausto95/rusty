# Control Flow

```rust
let age = 27;

if age > 18 {
    println!("Is adult");
} else {
    println!("Is not adult");
}

let is_adult = |x| x > 18;

if is_adult(age) {
    println!("Is adult");
} else if age > 15 {
    println!("Youre almost there");
} else {
    println!("Give up bro");
}
```
