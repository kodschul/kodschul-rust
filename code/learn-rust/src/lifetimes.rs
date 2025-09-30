pub fn run() {
    // let r;

    // {
    //     let x = 10;

    //     r = &x;

    //     // println!("r: {r}");
    // }

    // println!("r: {r}");
}

// Compiler weiß nicht: „Welche Referenz lebt länger? a oder b?“
// fn longest_(a: &str, b: &str) -> &str {
//     if a.len() > b.len() {
//         a
//     } else {
//         b
//     }
// }

// LÖSUNG
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn foo(a: &str, b: &str) {}
// wird interpretiert als
fn foo_intern<'a, 'b>(a: &'a str, b: &'b str) {}
