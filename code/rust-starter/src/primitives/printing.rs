pub fn run() {
    println!("Hello World from Rust");

    println!("Position arg: {1} is here, {0}", 1, 2);

    let x = 5;
    let y = 10;

    let ans = x * y;
    let ans_text = format!("{x} * {y} = {ans}");

    // println!("{:?}", ans_text);
    println!("{ans_text}");

    let tuple = (123, "Test", 10.5);

    println!("{:?}", tuple);
    println!("{:#?}", tuple);

    // ES GEHT NICHT
    // println!(abc);
}
