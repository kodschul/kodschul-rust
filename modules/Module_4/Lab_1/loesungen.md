# Arbeiten mit Cargo & Crates.io – Lösungen

## Aufgabe 1

1. Öffne ein Terminal und erstelle das Projekt:
```bash
cargo new dice
cd dice
```
2. Ersetze den Inhalt von `src/main.rs` durch:
```rust
fn main() {
    println!("Hallo, Rust!");
}
```
3. Baue und führe das Programm:
```bash
cargo run
```
Es sollte "Hallo, Rust!" ausgeben.


## Aufgabe 2

1. Öffne die Datei `Cargo.toml` und füge unter `[dependencies]` hinzu:
```toml
[dependencies]
rand = "0.8"
```
2. Passe `src/main.rs` an:
```rust
use rand::Rng;
fn main() {
    let n: i32 = rand::thread_rng().gen_range(1..=6);
    println!("Würfel zeigt: {}", n);
}
```
3. Führe das Programm erneut mit `cargo run` aus. Es sollte bei jedem Lauf eine Zufallszahl zwischen 1 und 6 ausgeben.


## Aufgabe 3

1. Führe im Projektordner aus:
```bash
cargo build       # Debug‑Build
cargo build --release
```
2. Nach dem Debug‑Build liegt das Binary unter `target/debug/dice`, nach dem Release‑Build unter `target/release/dice`.
3. Die Release‑Version ist deutlich kleiner und schneller, da sie optimiert wurde. Du kannst die Dateigrößen mit `ls -lh target/debug/dice target/release/dice` vergleichen.


