pub fn run() {
    let vec1 = vec![4, 5, 2, -1, 6];
    let vec2: Vec<i64> = vec![9, 10, 9, 100, 4];

    let string_vec = vec!["abc", "abcdef", "ab"];

    println!("min: {}", min(&vec1));
    println!("min i64: {}", min(&vec2));
    println!("min str: {}", min(&string_vec));
}

fn min_i32(list: &[i32]) -> i32 {
    list[0]
}

fn min<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut min = list[0];

    for item in list {
        if item < &min {
            min = *item
        }
    }

    min
}
