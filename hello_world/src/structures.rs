struct Box {
    width: i32,
    height: i32,
}

pub fn create_box() {

    let b = Box {
        width: 10,
        height: 20,
    };

    let height = b.height;

    println!("width: {}, height: {:?}", b.width, height);
}

struct Person {
    name: String,
    age: u8,
}

pub fn create_person() {
    let name = String::from("Faustino");
    let age = 27;

    let p = Person { name, age };

    println!("name: {}, age: {}", p.name, p.age);
}


enum Flavours {
    Sparkling,
    Sweet,
    Fruity
}

struct Drink {
    name: String,
    flavour: Flavours,
    volume: f64,
}

pub fn create_drink() {
    let name = String::from("Coke");
    let flavour = Flavours::Sparkling;
    let volume = 0.5;

    let drink = Drink { name, flavour, volume };

    match drink.flavour {
        Flavours::Sparkling => println!("{} is sparkling", drink.name),
        Flavours::Sweet => println!("{} is sweet", drink.name),
        Flavours::Fruity => println!("{} is fruity", drink.name),
    }

    println!("{} is {}l", drink.name, drink.volume);

}
