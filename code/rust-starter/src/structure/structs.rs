struct Person {
    name: String,
    age: i32,
    city: String,
}

pub fn run() {
    let mut person1 = Person {
        name: "Franz".to_string(),
        age: 21,
        city: "Stuttgart".to_string(),
    };

    let mut person2 = Person {
        name: person1.name,
        age: person1.age,
        city: "Frankfurt".to_string(),
    };

    person1.name = "Franz Overridden".to_string();
    person1.age = 22;

    person2.name = "Mike".to_string();
    person2.age = 25;

    let random_person = &person2;

    match random_person {
        Person {
            age: 25,
            name: _name,
            city: _city,
        } => println!("Person ist 25"),
        Person {
            age: 22,
            name: _name,
            city: _city,
        } => println!("Person ist 22"),
        _ => println!("Person ist weder 22 noch 25"),
    };

    println!(
        "Person1: Hi {}, you're {} years old from {}",
        person1.name, person1.age, person1.city
    );

    println!(
        "Person2: Hi {}, you're {} years old from {}",
        person2.name, person2.age, person2.city
    );

    return;

    // let mut person2 = Person:("Franz", "Doe", 21);
    // println!(
    //     "Person2: Hi {}, you're {} years old from {}",
    //     person2.name, person1.age, person1.city
    // );
}
