use std::{thread, time::Duration};

pub fn run() {
    let numbers = vec![1, 2, 3, 4, 5];
    let mut handles = vec![];

    for n in numbers {
        handles.push(thread::spawn(move || {
            println!("single thread running: {n}");
            thread::sleep(Duration::from_secs(3));
            n * n
        }));
    }

    for h in handles {
        println!("output: {}", h.join().unwrap());
    }
}
