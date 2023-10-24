# Loop

Rust has multiple types of loops.

- Loop: infinite loop
- While: conditional loop

### Infinite loop

```rust
let mut a = 0;

loop {
    if a == 5 {
        break;
    }
    println!("{a:?}");
    a = a + 1;
}
```

### Conditional loop

```rust
let mut a = 0;

while a != 5 {
    println!("{a:?}");
    a = a + 1;
}
```
