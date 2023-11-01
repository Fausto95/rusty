# Options

It's used in scenarious where we might or might not have data.

The internal definition of `Option` is the following:

```rust
enum Option<T> {
    Some(T),
    None
}
```

