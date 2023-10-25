# Vectors 

Vectors is a list data structure, that allows us to store multiple pieces of data at once. 

Vectors are like arrays in other programming languages, but in Rust, it can only store data of the same type

### Creating a vector using the `vec!` macro

```rust
let numbers = vec![1, 2, 3];
let second = numbers[2];
```

### Creating a vector using the `Vect::new` from the Vect struct

```rust
let mut numbers = Vec::new();

numbers.push(1);
numbers.push(2);
numbers.push(3);

numbers.push(4);
numbers.pop();

let length = numbers.len();
let second = numbers[2];
```

### Looping through vectors

```rust
let numbers = vec![1, 2, 3];

for num in numbers {
    println!("{:?}", num);
}
```