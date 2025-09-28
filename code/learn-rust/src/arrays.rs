use std::mem;

pub fn run() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    for num in numbers {
        println!("Num: {num}");
    }

    println!("full: {:?}", numbers);
    println!("first: {:?}", numbers[0]);

    // capacities of arr
    println!("mem: {:?}", mem::size_of_val(&numbers));

    // slice
    println!("Slice: {:?}", &numbers[0..2]);

    let arr = [1; 5]; // Creates an array of five zeros
    println!("{:?}", arr); // Outputs: [0, 0, 0, 0, 0]
}
