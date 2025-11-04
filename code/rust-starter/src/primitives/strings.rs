pub fn run() {
    let mut s = String::from("hello world");
    s.push_str(", this is the other part");

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

    // string with a minimum fixed capacity

    let mut str_with_reserve_storage = String::with_capacity(10);

    str_with_reserve_storage.push('a');
    str_with_reserve_storage.push('b');
    str_with_reserve_storage.push('c');

    str_with_reserve_storage.push_str("adasdasdsadsada");

    // assert_eq!(2, str_with_reserve_storage.len());
    // assert_eq!(10, str_with_reserve_storage.capacity());
}
