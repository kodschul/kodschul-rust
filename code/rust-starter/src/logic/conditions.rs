pub fn run() {
    let code = 4;

    if code == 1 {
        println!("Code is one")
    } else if code == 2 {
        println!("Code is two")
    } else if code == 3 {
        println!("Code is three")
    } else {
        println!("Code is neither 1, 2 nor 3")
    }

    let code_status_text = if code % 2 == 0 { "gerade" } else { "ungerade" };
    println!("Code ist {code_status_text}");

    match code {
        _ if code < 0 => println!("Code is negative"),
        1 => println!("Code is one"),
        2 => println!("Code is two"),
        3 => println!("Code is three"),
        4..10 => println!("Code is between 4 and 9"),
        4..=10 => println!("Code is between 4 and 10"),
        _ => println!("Code is not in range"),
    }

    let code_output = match code {
        1..5 => "zwischen 1 und 4",
        5..10 => "zwischen 5 und 9",
        _ => "not in range",
    };

    println!("Match output: {code_output}");
}
