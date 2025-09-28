pub fn run() {
    let mut s = String::from("hello world");
    // let mut y = s;

    // s.push_str(", this is the other part");

    // let mut s_static = "hello word";
    // let mut s_static_copy = s_static;

    // s_static = " the other part";

    // println!("dynamic string: {y}");
    // println!("static: {s_static}");

    let hello = &s[0..5];
    println!("Hello: {}", hello);

    let str1 = "Abc test";

    // won't work
    // str1.push("A");

    let mut str2 = String::from("String1");

    // add a char
    str2.push('A');
    println!("{}", str2);

    // add a string
    str2.push_str(" New other part");
    println!("{}", str2);

    // size in bytes
    println!("Capacity: {}", str2.capacity());

    // is empty?
    println!("Empty: {0} {1}", "".is_empty(), str2.is_empty());

    // check if contains
    println!("Contains A: {}", str2.contains("A"));

    let _str_words = str2.split_whitespace();

    // for x in str_words {
    //     println!("{}", x)
    // }

    println!("Len: {}", str1.len());

    // string with a fixed capacity

    let mut limited_str = String::with_capacity(10);

    limited_str.push('a');
    limited_str.push('b');
    limited_str.push('c');

    limited_str.push_str("adasdasdsadsada");

    // assert_eq!(2, limited_str.len());
    // assert_eq!(10, limited_str.capacity());
}
