pub fn run() {
    let text1 = "abc";
    let text2 = "abcdef";

    let res = longest(text1, text2);

    println!("text1: {text1}, text2: {text2}");
    println!("The longest string is: {res}");
}

pub fn run_three() {
    let numbers = vec![1, 2, 3, 4, 5];
    let numbers_two = [1, 2, 3, 4, 5];

    let nums = vec_len(&numbers);
    println!("Nums: {nums}");

    println!("Vec: {:?}", numbers);
}

fn arr_len(arr: [i32; 5]) -> usize {
    arr.len()
}

fn vec_len(arr: &Vec<i32>) -> usize {
    arr.len()
}
pub fn run_two() {
    // no ownership or borrowing, stack memory
    let t1 = "hallo";
    let t2 = t1;

    println!("{t1}");
    println!("{t2}");

    let x: Option<String> = None;
    println!("initial none: {:?}", x);

    // with ownership and borrowing, heap memory
    let mut s1 = String::from("hallo");
    s1 = "hallo2".to_string();
    let s2 = takes_and_gives_back(s1);

    // println!("s1: {s1}");
    // println!("s2: {s2}");

    let new_str = gives_ownership();

    takes_ownership(new_str);
    // print!("Try to get new_str should failed: {new_str}");

    let new_str2 = gives_ownership();
    let new_str3 = takes_and_gives_back(new_str2);

    // print!("Try to get new_str should failed: {new_str2}");
    println!("Try to get new_str after takes_and_gives_back should work: {new_str3}");

    println!("-----------");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// fn longest2(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn gives_ownership() -> String {
    let my_str = String::from("my_str");
    my_str
}

fn takes_ownership(in_str: String) {
    println!("Take ownership of: {in_str}");
}

fn takes_and_gives_back(in_str: String) -> String {
    in_str
}
