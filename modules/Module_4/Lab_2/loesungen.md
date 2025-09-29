# Tools, die helfen: rustfmt, clippy & rustdoc – Lösungen

## Aufgabe 1

1. Erstelle eine Datei `src/main.rs` mit folgendem Inhalt:
```rust
fn main(){let x=3;let y=4;println!("{} * {} = {}",x,y,x*y);}
```
2. Führe `cargo fmt` aus. Der Code wird zu:
```rust
fn main() {
    let x = 3;
    let y = 4;
    println!("{} * {} = {}", x, y, x * y);
}
```
3. **Unterschiede:** Einrückungen, Leerzeichen und Zeilenumbrüche werden nach den offiziellen Stilregeln korrigiert, was die Lesbarkeit erhöht.


## Aufgabe 2

1. Beispielprogramm (`src/main.rs`):
```rust
fn main() {
    let vec = vec![1, 2, 3];
    if vec.len() > 0 {
        println!("Nicht leer");
    }
    let v2 = vec.clone();
    println!("Anzahl Elemente: {}", v2.len());
}
```
2. Führe `cargo clippy` aus. Clippy empfiehlt, `!vec.is_empty()` statt `vec.len() > 0` zu verwenden und warnt, dass `vec.clone()` hier unnötig ist, weil `vec` danach nicht mehr verwendet wird.
3. Korrigierte Version:
```rust
fn main() {
    let vec = vec![1, 2, 3];
    if !vec.is_empty() {
        println!("Nicht leer");
    }
    // Kein clone nötig, wir können vec direkt benutzen
    println!("Anzahl Elemente: {}", vec.len());
}
```


## Aufgabe 3

1. Füge in `src/lib.rs` die folgende Funktion ein:
```rust
/// Addiert zwei i32‑Werte und gibt das Ergebnis zurück.
///
/// # Beispiele
///
/// ```
/// assert_eq!(my_crate::add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```
2. Erstelle eine `Cargo.toml` mit `lib`‑Sektion (oder nutze ein bestehendes Projekt) und führe `cargo doc --open` aus. Es öffnet sich eine HTML‑Dokumentation im Browser, die die Beschreibung und Beispiele anzeigt.
3. **Vorteile:** rustdoc ermöglicht eine standardisierte Dokumentation direkt aus dem Quellcode. Beispiele können als Tests ausgeführt werden (`cargo test`). Die generierte HTML‑Doku erleichtert es, APIs zu verstehen und zu nutzen.


