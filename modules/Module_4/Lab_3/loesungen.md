# Best Practices für den Alltag – Lösungen

## Aufgabe 1

```rust
fn might_fail() -> Result<i32, &'static str> { Ok(42) }
fn main() {
    // Unidiomatisch
    let result = might_fail();
    if result.is_ok() {
        println!("Wert: {}", result.unwrap());
    }
    // Idiomatisch
    if let Ok(val) = might_fail() {
        println!("Wert: {}", val);
    }
    // Alternative mit ? in einer Funktion, die Result zurückgibt
}
```
**Erläuterung:** `is_ok()` und `unwrap()` führen zu einem Panic, falls ein Fehler auftritt. `if let Ok(val)` behandelt nur den Erfolgsfall und ignoriert elegant den Fehler. In Funktionen, die selbst ein `Result` zurückgeben, kann der `?`‑Operator verwendet werden, um Fehler nach außen zu propagieren.


## Aufgabe 2

```rust
fn sum_vec(nums: &[i32]) -> i32 {
    nums.iter().sum()
}
fn main() {
    let numbers = vec![1, 2, 3, 4];
    let total = sum_vec(&numbers);
    println!("Summe: {}", total);
    // numbers kann weiterhin verwendet werden, da nur ein Slice übergeben wurde
    println!("Erstes Element: {}", numbers[0]);
}
```
**Erläuterung:** Anstatt den ganzen Vektor zu klonen, wird eine Slice‑Referenz (`&[i32]`) übergeben. Dadurch bleibt der Besitzer erhalten und es entstehen keine unnötigen Kopien.


## Aufgabe 3

**Verzeichnisstruktur:**
```
my_project/
├─ Cargo.toml
├─ src/
│  ├─ main.rs
│  ├─ lib.rs
│  ├─ utils.rs
├─ tests/
│  └─ integration_test.rs
├─ benches/
│  └─ benchmark.rs
├─ examples/
│  └─ usage.rs
```
**Anleitung:**
```bash
cargo new my_project
cd my_project
# Erstelle lib.rs und utils.rs
echo "pub fn util() {}" > src/lib.rs
echo "pub fn helper() {}" > src/utils.rs
mkdir tests benches examples
echo "#[test] fn it_works() {}" > tests/integration_test.rs
echo "fn main() {}" > benches/benchmark.rs
echo "fn main() {}" > examples/usage.rs
```
Diese Struktur trennt das ausführbare Programm (`main.rs`) von der wiederverwendbaren Bibliothek (`lib.rs`) und stellt Platz für Tests, Benchmarks und Beispiele bereit.


