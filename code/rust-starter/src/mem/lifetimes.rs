pub fn run() {
    {
        let vec1: Vec<i32> = vec![4, 5, 2, 6];
        let longest_vec;

        let vec2: Vec<i32> = vec![9, 10, 9, 100, 4];
        longest_vec = longest(&vec1, &vec2);
        println!("{:?}", longest_vec);
    }
}

fn longest<'a, 'b>(v1: &'a Vec<i32>, v2: &'a Vec<i32>) -> &'a Vec<i32> {
    if v1.len() > v2.len() {
        v1
    } else {
        v2
    }
}
