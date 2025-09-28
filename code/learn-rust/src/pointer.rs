pub fn run() {
    let mut x = 50;
    let y = &mut x;
    *y = 100;

    // let s1 = String::from("test");

    // let s2 = s1.clone();

    // x = 20;

    let z = &y;

    println!("x: {}", y);
    // println!("y: {}", y);
}
