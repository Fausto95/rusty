# Macros

Macros expand into additional code
`println` Prints(displays) information to the terminal, it is useful for debugging.

```rust
let name = "Faustino";
println!("{}", name);
println!("{:?}", name);
println!("{:?} {:?}", name, name);
println!("Hello, my name is {:?}", name);

println!("Hello, my name is {name:?}"); // for debugging
println!("Hello, my name is {name}");
```

`:?` is for debugging. Using a variable direclty is the value end users see

- Macros use an exclamation point to call/invoke
- Generate additional Rust code
- Data can be printed using println!:
    - {:?}
    - {varname:?}
    - {varname}
