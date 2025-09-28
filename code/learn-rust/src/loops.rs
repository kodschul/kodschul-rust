pub fn run() {
    'main_loop: loop {
        println!("Main loop running....");

        println!("Count 1 to 10");
        let mut i = 0;
        loop {
            i += 1;

            println!("{i}");

            if i == 5 {
                break 'main_loop;
            }

            if i == 10 {
                break;
            }
        }
    }
}
