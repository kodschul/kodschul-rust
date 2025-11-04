pub fn run() {
    let mut n = 0;

    loop {
        if n == 2 {
            break;
        }

        println!("loop n = {n}");
        n += 1;
    }

    let mut n = 0;

    while n <= 3 {
        println!("while n = {n}");
        n += 1;
    }

    for i in 0..3 {
        println!("for i = {i}");
    }
}
