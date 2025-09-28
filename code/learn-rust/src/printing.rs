pub fn run() {
    println!("Hello from print");

    // Printing integers
    // println!(1);
    println!("This has to be like {}", 1);

    // basic formatting
    println!("{} is a dev from {}", "Franz", "Stuttgart");

    // positional arguments
    println!(
        "{0} is a dev and {0} can coach also. He is from {1}",
        "Franz", "Stuttgart"
    );

    // named args
    println!(
        "{name} is a dev from {city}",
        city = "Stuttgart",
        name = "Franz"
    );

    // placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal {:0}", 10, 10, 10);

    // placeholder for debugs
    println!("{:?}", (123, "Test", "Test"));

    // pretty print
    println!("{:#?}", (123, "Test", "Test"));
}
