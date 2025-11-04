use std::collections::HashSet;

pub fn run() {
    let static_num_arr = [1, 2, 3, 4, 5];
    let mut numbers: Vec<i32> = Vec::new();
    // let mut numbers: Vec<i32> = vec![];
    let mut numbers: Vec<i32> = Vec::from(static_num_arr);

    numbers.push(2);
    numbers.push(5);
    numbers.pop();

    println!("full: {:?}", numbers);
    println!("first: {:?}", numbers[0]);

    // slice
    println!("Slice: {:?}", &numbers[0..2]);

    let arr = [0; 5]; // Creates an array of five zeros
    println!("{:?}", arr); // Outputs: [0, 0, 0, 0, 0]

    // iterate vectors
    for x in &mut numbers {
        *x = *x * 2;
    }

    println!("Final vec:  {:?}", numbers);
}
