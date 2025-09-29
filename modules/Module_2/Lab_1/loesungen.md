# Skalare Typen, Variablen & Kontrollfluss – Lösungen

## Aufgabe 1

```rust
fn main() {
    let age: i32 = 30;
    let temperature: f64 = 21.5;
    let logged_in: bool = true;
    let initial: char = 'A';
    println!("age={}, temperature={}, logged_in={}, initial={}", age, temperature, logged_in, initial);
}
```


## Aufgabe 2

```rust
fn sign(n: i32) -> &'static str {
    match n {
        x if x > 0 => "positiv",
        x if x < 0 => "negativ",
        _ => "null",
    }
}
fn main() {
    for n in [-5, 0, 4] {
        println!("{} ist {}", n, sign(n));
    }
}
```


## Aufgabe 3

```rust
fn main() {
    // loop
    let mut i = 1;
    loop {
        if i > 3 { break; }
        println!("{}", i);
        i += 1;
    }
    // while
    let mut j = 1;
    while j <= 3 {
        println!("{}", j);
        j += 1;
    }
    // for
    for k in 1..=3 {
        println!("{}", k);
    }
}
```
**Erläuterung:** `loop` ist eine unendliche Schleife mit manuellem Abbruch; `while` prüft eine Bedingung; `for` iteriert über einen Range und ist in Rust am gebräuchlichsten.


