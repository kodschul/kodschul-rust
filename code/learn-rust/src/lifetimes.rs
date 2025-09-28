pub fn run() {
    let r;

    {
        let x = 10;

        r = &x;

        // println!("r: {r}");
    }

    println!("r: {r}");
}
