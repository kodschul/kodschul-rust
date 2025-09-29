# Das Ownership-Modell – Theorie & Codebeispiele


**Thema:** Das Ownership‑Modell (Lab 3.1)

Rust verwaltet Speicher durch strikte Regeln, die garantieren, dass es genau einen Besitzer für jede Ressource gibt. Wenn der Besitzer den Scope verlässt, wird die Ressource freigegeben. Die Folien zeigen ein einfaches Beispiel: eine `String` wird in einer Funktion erstellt und am Ende des Gültigkeitsbereichs automatisch gelöscht. Weiterhin wird die **Move‑Semantik** demonstriert, bei der der Besitz beim Zuweisen oder Funktionsaufruf auf den neuen Namen übergeht. Nach dem Move ist der ursprüngliche Name ungültig und darf nicht mehr verwendet werden.

Der Unterschied zwischen Stack‑ und Heap‑Speicher wird erläutert: Primitive Werte wie `i32` werden kopiert (`Copy`‑Trait), während komplexere Typen wie `String` verschoben werden. Dieses Modell verhindert Datenrennen und doppelte Freigaben zur Compile‑Zeit【674319147427555†L140-L149】.

**Codebeispiele aus den Folien**
```rust
fn main() {
    let s = String::from("Hallo");
    takes_ownership(s); // s wird bewegt und ist danach ungültig
    let n = 5;
    makes_copy(n); // n ist noch gültig
}
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
fn makes_copy(some_int: i32) {
    println!("{}", some_int);
}
```

