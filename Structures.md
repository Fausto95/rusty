# Structure

it's a type that contains multiple pieces of data - All or nothing - can't have some pieces of data and not the others.

Each piece of data is called a `field`

Structs make working with data easier - similar data can be grouped together.

- Structs deal with multiple piece of data
- All fields must be present to create a struct
- Fields can be accessed using a dot notation `.`


```rust
struct Box {
    width: i32,
    height: i32,
}

fn main() {

    let b = Box {
        width: 10,
        height: 20,
    };

    let height = b.height;

    println!("width: {}, height: {:?}", b.width, height);
}
```