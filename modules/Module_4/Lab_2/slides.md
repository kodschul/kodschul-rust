# Tools, die helfen: rustfmt, clippy & rustdoc – Theorie & Codebeispiele


**Thema:** Tools: rustfmt, clippy & rustdoc (Lab 4.2)

Rust verfügt über hilfreiche Werkzeuge zur Code‑Qualität. **rustfmt** formatiert Quellcode nach offiziellen Stilregeln. Die Folien zeigen ein unformatiertes Programm und seine Ausgabe nach `cargo fmt`.

**clippy** ist ein Linter, der potenzielle Fehler und Stilprobleme erkennt. Ein Beispiel zeigt die Warnung, statt `vec.len() > 0` die Methode `is_empty()` zu verwenden und unnötige `clone()`‑Aufrufe zu vermeiden. Weitere Hinweise helfen, idiomatischen Rust‑Code zu schreiben.

**rustdoc** generiert aus Dokumentationskommentaren (`///`) HTML‑Dokumentation. In den Folien wird eine Funktion `add` mit doc‑Kommentar versehen und es wird gezeigt, wie `cargo doc --open` die generierten Dokumente im Browser öffnet.

**Codebeispiele aus den Folien**
```rust
// Vor rustfmt
fn main(){let x=5;let y=10;println!("{} + {} = {}",x,y,x+y);}
// Nach cargo fmt
fn main() {
    let x = 5;
    let y = 10;
    println!("{} + {} = {}", x, y, x + y);
}

// clippy Beispiel
fn main() {
    let vec = vec![1, 2, 3];
    if vec.len() > 0 {
        println!("Nicht leer");
    }
}
// clippy schlägt !vec.is_empty() vor und warnt vor unnötigen clone()s

// rustdoc Beispiel
/// Addiert zwei Zahlen.
///
/// # Beispiele
///
/// ```
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

