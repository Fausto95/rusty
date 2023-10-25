# Tuples 

A tuple is a type of record, it stores data anonymously, meaning that we dont need to name fields.

It's useful for returning data in pairs from functions.

We can destructure tuples into variables.

```rust
enum Nationality {
    Angolan,
}

fn make_tuple() -> (i32, i8, u32) {
    (10_000, 27, 2_000)
}

fn main() {
    let result = make_tuple();
    let (a, b, c) = make_tuple();

    println!("{:?}, {:?}", a, result.0);
    println!("{:?}, {:?}", b, result.1);
    println!("{:?}, {:?}", c, result.2);

    //let (name, nationality) = ("Faustino", Nationality::Angolan);
}

```