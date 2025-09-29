# Rust in der Praxis: Was kann ersetzt werden? – Lösungen

## Aufgabe 1

```rust
fn average(nums: &[i32]) -> f64 {
    let sum: i32 = nums.iter().sum();
    sum as f64 / nums.len() as f64
}
fn main() {
    let values = vec![10, 20, 30, 40];
    println!("Durchschnitt: {}", average(&values));
}
```
**Erläuterung:** In C müsste man Speicher selbst verwalten. In Rust übernimmt `Vec` die Speicherverwaltung. Funktionen nehmen in der Regel eine Slice `&[T]`, um unnötige Kopien zu vermeiden.


## Aufgabe 2

```rust
fn get_element(data: &[u8], index: usize) -> Option<u8> {
    data.get(index).copied()
}
fn main() {
    let bytes = [1u8, 2, 3];
    println!("{:?}", get_element(&bytes, 1)); // Some(2)
    println!("{:?}", get_element(&bytes, 5)); // None
}
```
**Erläuterung:** Statt roher Zeiger verwendet man in Rust Slices (`&[T]`), die Länge und Typinformationen enthalten. Mit `get` wird der Index geprüft und `Option` signalisiert einen möglichen Ausfall.


## Aufgabe 3

**Mögliche Antwort:**

* **Treiber und Betriebssystem‑nahe Module:** Hier treten häufig Pufferüberläufe und NULL‑Pointer‑Dereferenzen auf. Rust verhindert solche Fehler durch sein Ownership‑Modell und Borrowing.
* **Kryptographie‑Bibliotheken:** Sicherheit ist kritisch; Rust bietet Speichersicherheit ohne Garbage Collector und stellt sicher, dass sensible Daten nicht unabsichtlich kopiert werden.
* **Netzwerk‑Stack oder Parser:** Durch strenge Typprüfung verhindert Rust undefiniertes Verhalten und erleichtert nebenläufige Programmierung. Dank `Result`/`Option` werden Fehler explizit behandelt.
* **Parallelisierte Algorithmen:** Das Typsystem von Rust verhindert Datenrennen zur Compile‑Zeit. Im Vergleich zu C++ ist der Code oft leichter zu überprüfen.


