struct Person {
    name: String,
    age: i8,
}

impl Person {
    fn new(name: String, age: i8) -> Self {
        Self {
            name,
            age,
        }
    }

    fn is_adult(&self) -> bool {
        self.age >= 18
    }
}

pub fn log_person() {
    let person = Person {
        name: String::from("John"),
        age: 25,
    };

    println!("xx Is adult: {}", person.is_adult());
    
    let new_person: Person = Person::new(String::from("Fausto"), 28);
    
    println!("xx Is adult: {}", new_person.is_adult());
}