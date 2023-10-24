pub fn repetition() {
    let mut count = 10;
    loop {
        if count <= 0 {
            break;
        }
        println!("{}", count);
        count -= 1;
    }
    println!("done counting!");
}

pub fn while_loop() {
    let mut count = 10;
    while count > 0 {
        println!("{}", count);
        count -= 1;
    }
    println!("done counting!");
}