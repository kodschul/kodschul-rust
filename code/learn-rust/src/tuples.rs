pub fn run() {
    let country: (&str, &str, i8) = ("Cameroon", "Ydé", 1);

    println!(
        "The capital of {} is {} and is ranked #{}",
        country.0, country.1, country.2
    );

    let nested_tuple = ((1, 2), "Nested", (3.14, 'x'));
    println!("{:?}", nested_tuple); // Outputs: ((1, 2), "Nested", (3.14, 'x'))
                                    // Accessing nested elements
    println!("First of first tuple: {}", nested_tuple.0 .0); // Outputs: 1

    // Destructing tuples
    let my_tuple = (42, 3.14, "Hello");
    let (x, y, z) = my_tuple;
    println!("x: {}, y: {}, z: {}", x, y, z); // Outputs: x: 42, y: 3.14, z: Hello
}
