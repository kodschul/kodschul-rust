// mod printing;
// mod variables;
// mod strings;
// mod tuples;
// mod arrays;
// mod vectors;
// mod conditions;
// mod pointer;
// mod functions;
// mod structs;
// mod ownership;
// mod traits;
// mod macros;
// mod loops;
// mod lifetimes;
// mod enums;
// mod c_modules;
// mod lifetimes;
// mod ownership_ref;
// mod run_c;
// mod traits;

// import only a single func from a mod
// use crate::structs::run;

// use num_bigint::BigUint;

mod cpp;

fn main() {
    // variables::run();
    // printing::run();
    // strings::run();
    // tuples::run();
    // arrays::run();
    // vectors::run();
    // pointer::run();
    // functions::run();
    // conditions::run();
    // structs::run();
    // loops::run();
    // lifetimes::run();

    // ownership::run();
    // traits::run();

    // macros::run();

    // run_c::main();

    let a = 10;
    let b = 5;
    let res = cpp::cpp::add(a, b);

    println!("Result = {res}");

    // ownership_ref::main();

    // let x: BigUint = "10".parse().unwrap();
    // println!("This is the value of x: {:#?}", x);
}
