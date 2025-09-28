pub fn run() {
    let ans = add(5, 5);
    println!("5 + 5 = {}", ans);

    let get_sum = |x: i32, y: i32| x + y;
    println!("Closure: 2 + 4 = {:?}", get_sum(2, 4));
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn add2(x: i32, y: i32) -> i32 {
    return x + y;
}

// closure | anonym func
