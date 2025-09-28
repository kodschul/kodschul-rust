macro_rules! say_hello {
    () => {
        fn hello() {
            println!("Hello World!");
        }
    };
}

macro_rules! personalized_hello {
    ($name: expr) => {
        println!("Hello, {}!", $name);
    };
}

say_hello!();

pub fn run() {
    hello();

    personalized_hello!("Franz");
}
