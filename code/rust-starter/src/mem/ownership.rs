pub fn run() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("r1 = {r1}, r2= {r2}");

    s.push_str(" world");

    // println!("r1 ={r1}");

    // let w1 = &mut s;
    // w1.push_str(" world");

    // println!("w1 ={w1}");
    println!("s ={s}");
}
