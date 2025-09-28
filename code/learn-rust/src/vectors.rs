use core::num;
use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    numbers.push(2);
    numbers.push(5);
    numbers.pop();

    println!("full: {:?}", numbers);
    println!("first: {:?}", numbers[0]);

    // capacities of arr
    println!("mem: {:?}", mem::size_of_val(&numbers));

    // slice
    println!("Slice: {:?}", &numbers[0..2]);

    let arr = [0; 5]; // Creates an array of five zeros
    println!("{:?}", arr); // Outputs: [0, 0, 0, 0, 0]

    // iterate vectors

    for x in numbers.iter_mut() {
        *x *= 2;
        println!("{}", x);
    }
    println!("Final vec:  {:?}", numbers);
}
