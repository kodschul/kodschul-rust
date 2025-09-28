struct Person {
    name: String,
    age: i32,
    city: String,
}

enum TodoStatus {
    ToDo,
    InProgress,
    Done,
}

impl Person {
    fn new(first: &str, last: &str, age: i32) -> Person {
        Person {
            name: first.to_string() + " " + last,
            age: age,
            city: "Unknown".to_string(),
        }
    }

    fn greet(self) {
        println!(
            "self: Hi {}, you're {} years old from {}",
            self.name, self.age, self.city
        );
    }

    fn set_age(&mut self, new_age: i32) {
        self.age = new_age;
    }
}

pub fn run() {
    let mut person1 = Person {
        name: "Franz".to_string(),
        age: 21,
        city: "Stuttgart".to_string(),
    };

    person1.name = "Franz Overridden".to_string();

    println!(
        "Person1: Hi {}, you're {} years old from {}",
        person1.name, person1.age, person1.city
    );

    let mut person2 = Person::new("Franz", "Doe", 21);
    person2.set_age(22);
    person2.greet();
    // println!(
    //     "Person2: Hi {}, you're {} years old from {}",
    //     person2.name, person1.age, person1.city
    // );

    return;

    let status = TodoStatus::ToDo;

    let current_status_str = match status {
        TodoStatus::ToDo => "to_do",
        TodoStatus::InProgress => "in_progress",
        TodoStatus::Done => "done",
    };

    println!("Current status: {}", current_status_str);
}
