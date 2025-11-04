pub fn run() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    for num in numbers {
        println!("Num: {num}");
    }

    println!("full: {:?}", numbers);
    println!("first: {:?}", numbers[0]);
    println!("length: {:?}", numbers.len());

    // slice
    println!("Slice: {:?}", &numbers[2..=3]);

    let arr = [1; 10]; // Creates an array of five zeros
    println!("{:?}", arr); // Outputs: [0, 0, 0, 0, 0]
}
