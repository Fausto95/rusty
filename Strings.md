# Strings

String are used to store text information.

There are two commonly used string types in Rust:

- String -> Owned
- &str -> Borrowed string slice

We must use an owned `String` to store in a struct, and use `&str` when passing to a function.


```rust
fn greet(name: &str) {
    println!("Hi {name:?}");
}

fn main() {
    greet("Faustino");

    let automatically_borrowed_string = "_Fausto_";

    let owned_str = "Fausto".to_owned();
    let owned_str_bis = String::from("Faust");

    greet(automatically_borrowed_string);
    greet(&owned_str);
    greet(&owned_str_bis);
}
```

To save strings within a `struct` we need to use the owned `String`. 

```rust
struct Person {
    name: &str, // this won't work
    age: i8,
}

```

But we can call the `to_owned()` method on a borrowed string so that we can use it within a `struct`.


```rust
struct Person {
    name: String,
    age: i8,
}

fn main() {
    let name = "Faustino".to_owned();

    let person = Person {
        name,
        age: 27,
    };

    println!("My name is {:?} and am {:?} years old", person.name, person.age);
}

```