# Arbeiten mit Cargo & Crates.io – Theorie & Codebeispiele


**Thema:** Arbeiten mit Cargo & Crates.io (Lab 4.1)

Cargo ist das Build‑System und Paketmanager von Rust. Die Folien erklären den Befehl `cargo new`, der ein neues Projekt mit `Cargo.toml` und einem `src`‑Ordner erstellt【547768925550910†L176-L183】. In der Konfigurationsdatei `Cargo.toml` können Abhängigkeiten deklariert werden; beispielsweise fügt `[dependencies] rand = "0.8"` die Bibliothek `rand` hinzu【547768925550910†L192-L219】.

Ein Codebeispiel nutzt `rand` zum Generieren einer Zufallszahl zwischen 1 und 6:
```rust
use rand::Rng;
fn main() {
    let n: i32 = rand::thread_rng().gen_range(1..=6);
    println!("Würfel zeigt: {}", n);
}
```
Außerdem zeigen die Folien das Verzeichnislayout eines neuen Projekts (`src/main.rs`, `Cargo.toml`) und erklären, wie man mit `cargo build`, `cargo run` und `cargo build --release` unterschiedliche Builds ausführt.

