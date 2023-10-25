# Implementation

`impl` allows us to implement functionality on specific `struct` and `enum`.

```rust
struct Person {
    name: String,
    age: i8,
}

fn is_adult(person: &Person) -> bool {
    person.age >= 18
}

fn main() {
    let person = Person {
        name: String::from("John"),
        age: 25,
    };

    println!("Is adult: {}", is_adult(&person));
}

```

We can see that `is_adult` only makes sense for the person struct.

```rust
struct Person {
    name: String,
    age: i8,
}

impl Person {
    fn is_adult(person: &Person) -> bool {
        person.age >= 18
    }
}

fn main() {
    let person = Person {
        name: String::from("John"),
        age: 25,
    };

    println!("Is adult: {}", Person::is_adult(&person));
}
```

We can go a bit further

```rust
struct Person {
    name: String,
    age: i8,
}

impl Person {
    fn is_adult(&self) -> bool {
        self.age >= 18
    }
}

fn main() {
    let person = Person {
        name: String::from("John"),
        age: 25,
    };

    println!("Is adult: {}", person.is_adult());
}
```