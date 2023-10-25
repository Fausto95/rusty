pub fn expressions() {
    let age = 27;
    let is_adult = if age >= 18 { true } else { false };
    let is_adult = age >= 18;

    let is_adult = match age {
        0 => false,
        1..=12 => false,
        13..=19 => true,
        _ => true,
    };

    println!("is adult: {}", is_adult);
}

enum Access {
    Admin,
    User,
    Guest,
}

pub fn get_access_level() {
    let access_level = Access::Guest;

    let can_access_files = match access_level {
        Access::Admin => true,
        _ => false,
    };

    println!("can access files: {}", can_access_files);
}