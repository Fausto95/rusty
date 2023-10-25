struct Framework {
    name: String,
}

fn print(name: &str) {
    println!("Name: {}", name);
}

pub fn print_frameworks() {
    let frameworks = vec![
        Framework { name: String::from("React") },
        Framework { name: String::from("Vue") },
        Framework { name: "Solid".to_owned() },
    ];

    for framework in frameworks {
        print(&framework.name);
    }
}