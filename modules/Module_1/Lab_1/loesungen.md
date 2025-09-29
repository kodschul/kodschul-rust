# Sicherheit ohne Garbage Collector – Lösungen

## Aufgabe 1

```rust
fn main() {
    let s = String::from("Willkommen bei Rust");
    println!("{}", s);
    // Am Ende von main wird s automatisch freigegeben.
    // Durch das Ownership‑Modell besitzt `s` den Speicher. Sobald `s` den Scope verlässt,
    // ruft der Compiler automatisch die `drop`‑Funktion auf und gibt den Speicher frei.
}
```
**Erläuterung:** In Rust besitzt jede Ressource genau einen Eigentümer. Wenn der Eigentümer den Gültigkeitsbereich verlässt, wird die Ressource automatisch freigegeben. Dieses Konzept nennt sich *RAII*. Dadurch wird Speicherleckage vermieden und es ist kein Garbage Collector notwendig【674319147427555†L140-L149】.


## Aufgabe 2

```rust
fn main() {
    let a = String::from("Original");
    let b = a; // b übernimmt den Besitz von a
    // println!("{}", a); // dies würde einen Kompilierfehler verursachen
    println!("{}", b);
}
```
**Erläuterung:** Beim Zuweisen eines Heap‑basierten Werts wie `String` an eine neue Variable wird der Besitz verschoben (Move). Danach ist der ursprüngliche Name ungültig und kann nicht mehr verwendet werden. So verhindert Rust versehentliche Doppelfreigaben und Use‑After‑Free‑Fehler.


## Aufgabe 3

```rust
fn double_positive(x: i32) -> Option<i32> {
    if x > 0 { Some(x * 2) } else { None }
}
fn main() {
    let n = -3;
    match double_positive(n) {
        Some(val) => println!("Ergebnis: {}", val),
        None => println!("{} ist nicht positiv", n),
    }
}
```
**Erläuterung:** Die Verwendung von `Option<T>` anstelle von `null` zwingt den Aufrufer, beide Fälle zu behandeln. So wird ein potenzieller Nullzeigerzugriff verhindert【674319147427555†L221-L229】.


