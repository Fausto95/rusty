fn dispay_hello() {
    println!("Hello, world!");
}

fn display_goodbye() {
    println!("Goodbye, world!");
}

pub fn control_flow() {
    let age = 27;
    let is_adult = if age >= 18 { true } else { false };

    if is_adult {
        println!("You are an adult");
    } else if age < 18 && age >= 13 {
        println!("You are a teenager");
    } else {
        println!("You are a child");
    }

    let should_display_hello = true;

    if should_display_hello {
        dispay_hello();
    } else {
        display_goodbye();
    }
}

