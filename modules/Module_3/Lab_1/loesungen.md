# Das Ownership-Modell – Lösungen

## Aufgabe 1

```rust
fn print_and_consume(s: String) {
    println!("{}", s);
}
fn main() {
    let greeting = String::from("Hallo Welt");
    print_and_consume(greeting);
    // greeting wäre hier nicht mehr gültig:
    // println!("{}", greeting); // Fehler
    // Lösung: statt Ownership zu übernehmen, nur ein Borrow übergeben oder die Funktion gibt den String zurück.
}
```
**Erläuterung:** Die Funktion `print_and_consume` übernimmt den Besitz des Strings. Nach dem Aufruf ist die Variable `greeting` ungültig. Man kann entweder eine Referenz übergeben (`&String`) oder den Wert zurückgeben, um weiter darauf zuzugreifen.


## Aufgabe 2

```rust
fn take_and_return(s: String) -> String {
    println!("Gefangen: {}", s);
    s
}
fn main() {
    let word = String::from("Rust");
    let word = take_and_return(word);
    println!("Wieder verfügbar: {}", word);
}
```
**Erläuterung:** Durch Rückgabe des Besitzers kann der Aufrufer den Wert weiterverwenden. Alternativ könnte man eine Referenz nutzen, um keinen Besitztransfer durchzuführen.


## Aufgabe 3

```rust
fn use_value(x: i32) {
    println!("Inside: {}", x);
}
fn main() {
    let n = 10;
    use_value(n);
    // n ist weiterhin gültig, da i32 das Copy‑Trait implementiert
    println!("Außerhalb: {}", n);
}
```
**Erläuterung:** Primitive Typen implementieren das `Copy`‑Trait. Beim Funktionsaufruf wird eine Kopie des Werts erstellt, sodass der originale Wert im Aufrufer erhalten bleibt. Komplexe Typen wie `String` besitzen dieses Trait nicht und werden deshalb verschoben.


