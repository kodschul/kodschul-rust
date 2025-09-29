# Datenstrukturen mit Systemnähe – Lösungen

## Aufgabe 1

```rust
use std::f64;
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn distance_from_origin(&self) -> f64 {
        let xsq = (self.x as f64).powi(2);
        let ysq = (self.y as f64).powi(2);
        (xsq + ysq).sqrt()
    }
}
fn main() {
    let p = Point { x: 3, y: 4 };
    println!("Abstand: {}", p.distance_from_origin());
}
```


## Aufgabe 2

```rust
use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 10);
    scores.insert(String::from("Bob"), 5);
    scores.insert(String::from("Carol"), 8);
    scores.insert(String::from("Dave"), 12); // neuer Spieler
    // Score aktualisieren
    if let Some(score) = scores.get_mut("Bob") {
        *score += 3;
    }
    for (player, score) in &scores {
        println!("{}: {} Punkte", player, score);
    }
}
```


## Aufgabe 3

```rust
use std::collections::HashSet;
fn main() {
    let mut fruits = HashSet::new();
    fruits.insert("Apfel".to_string());
    fruits.insert("Banane".to_string());
    fruits.insert("Apfel".to_string());
    println!("Set enthält {} Elemente", fruits.len()); // 2
    println!("Enthält Apfel? {}", fruits.contains("Apfel"));
    println!("Enthält Kirsche? {}", fruits.contains("Kirsche"));
}
```


