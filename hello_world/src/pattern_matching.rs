pub fn pattern_matching() {
    let is_adult = false;

    match is_adult {
        true => println!("is adult"),
        false => println!("is not adult"),
    }

    let age = 27;

    match age {
        0 => println!("You are a baby"),
        1..=12 => println!("You are a child"),
        13..=19 => println!("You are a teenager"),
        _ => println!("You are an adult"),
    }
}