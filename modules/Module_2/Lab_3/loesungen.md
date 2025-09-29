# Funktionen, Traits und Generics – Lösungen

## Aufgabe 1

```rust
fn square(n: i32) -> i32 { n * n }
fn main() {
    println!("square(3) = {}", square(3));
    println!("square(-4) = {}", square(-4));
}
```


## Aufgabe 2

```rust
trait Printable {
    fn print(&self);
}
impl Printable for i32 {
    fn print(&self) { println!("i32: {}", self); }
}
impl Printable for String {
    fn print(&self) { println!("String: {}", self); }
}
fn show<T: Printable>(item: T) {
    item.print();
}
fn main() {
    show(42);
    show(String::from("Hallo"));
}
```


## Aufgabe 3

```rust
fn min<T: PartialOrd>(list: &[T]) -> &T {
    let mut smallest = &list[0];
    for item in list.iter() {
        if item < smallest { smallest = item; }
    }
    smallest
}
fn main() {
    let nums = vec![4, 1, 9, 2];
    println!("Min int: {}", min(&nums));
    let chars = vec!['z', 'a', 'm'];
    println!("Min char: {}", min(&chars));
}
```


