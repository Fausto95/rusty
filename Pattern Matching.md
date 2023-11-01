# Pattern matching

Pattern matching is like if/else statement, except that it needs to be exhaustive - meaning that we need to check for every single possible value.


```rust
fn main() {
    let is_adult = false;

    match is_adult {
        true => println!("is adult"),
        false => println!("is not adult"),
    }
}
```

Pattern matching also supports ranges

```rust
fn main() {

    let age = 27;

    match age {
        0 => println!("You are a baby"),
        1..=12 => println!("You are a child"),
        13..=19 => println!("You are a teenager"),
        _ => println!("You are an adult"),
    }
}
```


### Struct Matching

```rust
struct City {
    name: String;
}

let city = City {
    name: "Luanda".to_owned(),
}

match city {
    City {name} => println!("City is {name:?}"),
}
```