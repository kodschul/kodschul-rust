fn min<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut smallest = list[0];
    for &item in list.iter() {
        if item < smallest {
            smallest = item;
        }
    }
    smallest
}
