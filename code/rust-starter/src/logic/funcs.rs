pub fn run() {
    let x = 10;
    let y = 25;

    let ans = add(x, y);
    println!("{x} + {y} = {ans}");

    let multiply = |a: i32, b: i32| a * b;

    let ans = multiply(x, y);
    println!("{x} * {y} = {ans}");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
