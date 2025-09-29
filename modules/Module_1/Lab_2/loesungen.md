# Performance wie C – aber sicher – Lösungen

## Aufgabe 1

```rust
fn main() {
    let nums = vec![3, 6, 1, 8];
    let sum_iter: i32 = nums.iter().map(|x| x * 2).sum();
    let mut sum_loop = 0;
    for x in &nums {
        sum_loop += x * 2;
    }
    println!("Summe (Iterator): {}", sum_iter);
    println!("Summe (for‑Schleife): {}", sum_loop);
}
```
Beide Varianten liefern dasselbe Ergebnis. Iteratoren sind oft ergonomischer, während `for`‑Schleifen mehr Kontrolle bieten.


## Aufgabe 2

```rust
fn decode(code: u8) -> &'static str {
    match code {
        1 => "Start",
        2 => "Stop",
        _ => "Unbekannt",
    }
}
fn main() {
    for c in [1u8, 2, 42] {
        println!("Code {}: {}", c, decode(c));
    }
}
```


## Aufgabe 3

```rust
fn main() {
    // loop
    let mut i = 1;
    loop {
        if i > 5 { break; }
        println!("{}", i);
        i += 1;
    }
    // while
    let mut j = 1;
    while j <= 5 {
        println!("{}", j);
        j += 1;
    }
    // for
    for k in 1..=5 {
        println!("{}", k);
    }
}
```
**Erläuterung:** Eine `loop` läuft unendlich und muss manuell mit `break` beendet werden. `while` prüft die Bedingung am Schleifenanfang. `for` iteriert elegant über einen Range oder Iterator und ist in Rust die idiomatischste Variante für bekannte Sequenzen.


