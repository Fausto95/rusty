pub fn arithmetic() {
    let sum = add(5, 10);
    let difference = substract(95, 4);
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {product:?}");
    println!("Quotient: {quotient}");
    println!("Remainder: {:?}", remainder);

}

fn substract(a: i32, b: i32) -> i32 {
    a - b
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}