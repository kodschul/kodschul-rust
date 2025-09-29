# Referenzen & Borrowing verstehen – Lösungen

## Aufgabe 1

```rust
fn main() {
    let mut s = String::from("Hallo");
    {
        let r1 = &s;
        let r2 = &s;
        println!("Immutable: {}, {}", r1, r2);
    } // r1 und r2 enden hier
    let r3 = &mut s;
    r3.push_str(" Rust");
    println!("Mutiert: {}", r3);
}
```
**Erläuterung:** Im ersten Block existieren zwei unveränderliche Referenzen. Solange diese leben, lässt der Borrow‑Checker keine mutable Referenz zu. Durch das Beenden des Blocks wird der Scope beendet und eine mutable Referenz ist erlaubt.


## Aufgabe 2

```rust
fn append_exclamation(s: &mut String) {
    s.push('!');
}
fn main() {
    let mut msg = String::from("Hallo");
    append_exclamation(&mut msg);
    println!("{}", msg);
}
```
**Erläuterung:** Funktionen können mutable Referenzen akzeptieren, um Daten zu verändern, ohne den Besitz zu übernehmen. Der Borrow‑Checker stellt sicher, dass während des Aufrufs keine anderen Referenzen auf `msg` existieren.


## Aufgabe 3

```rust
fn main() {
    let numbers = vec![10, 20, 30];
    match numbers.get(1) {
        Some(val) => println!("Wert an Index 1: {}", val),
        None => println!("Kein Element an diesem Index"),
    }
    match numbers.get(5) {
        Some(val) => println!("Wert an Index 5: {}", val),
        None => println!("Index 5 existiert nicht"),
    }
}
```
**Erläuterung:** `Vec::get` gibt eine `Option<&T>` zurück. Durch `match` werden sowohl der Erfolgs‑ als auch der Fehlerfall explizit behandelt.


