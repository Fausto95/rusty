struct City {
    name: String,
    population: Option<i64>,
}

pub fn get_cities() {
    let cities = vec![
        City {
            name: String::from("Luanda"),
            population: Some(553165),
        },
        City {
            name: String::from("Paris"),
            population: Some(210000),
        },
        City {
            name: String::from("Mbicoyo"),
            population: None,
        },
    ];

    for city in cities {
        match city.population {
            Some(population) => println!("{} has population {}", city.name, population),
            None => println!("{} population is unknown", city.name),
        }
    }
}

#[derive(Debug)]
pub struct Person {
    name: String,
    age: i32,
}

pub fn find_person(name: &str) -> Option<Person> {
    if name == "John" {
        Some(Person {
            name: String::from("John"),
            age: 32,
        })
    } else {
        None
    }
}