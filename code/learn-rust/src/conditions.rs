pub fn run() {
    let number = 10;

    let output = match number {
        x if x > 2 => "x is greater 2",
        x if x < 2 => "x is less than 2",
        x if x == 2 => "x = 2",
        _ => "Default",
    };

    println!("Output: {}", output);

    // loop range

    for x in 1..30 {
        println!("This is x: {}", x);
    }
}
