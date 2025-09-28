
# Rust Schulung: Ownership und Borrowing

## Das Konzept des Ownerships

**Ownership** ist ein zentraler Bestandteil der Speicherverwaltung in Rust. Jeder Wert in Rust hat einen "Besitzer", und es gibt klare Regeln, wie dieser Besitzer funktioniert.

### Die drei Regeln des Ownerships:
1. **Jeder Wert in Rust hat genau einen Besitzer**:
   Der Besitzer eines Wertes wird durch die Variable definiert, die den Wert hält.

2. **Es kann immer nur einen Besitzer geben**:
   Wenn ein Wert an eine andere Variable übergeben wird, wechselt der Besitzer.

3. **Der Wert wird freigegeben, wenn der Besitzer den Gültigkeitsbereich verlässt**:
   Wenn eine Variable den Scope verlässt, wird der Speicher automatisch freigegeben.

### Beispiel:
```rust
fn main() {
    let s1 = String::from("Hallo");
    let s2 = s1; // Ownership von s1 wird an s2 übertragen

    // println!("{}", s1); // Fehler: s1 ist nicht mehr gültig
    println!("{}", s2); // Ausgabe: Hallo
}
```

### Cloning von Werten:
Wenn ein Wert explizit kopiert werden soll, kann die Methode `.clone()` verwendet werden.
```rust
fn main() {
    let s1 = String::from("Hallo");
    let s2 = s1.clone(); // Erstellen einer tiefen Kopie

    println!("{}", s1); // Gültig
    println!("{}", s2); // Gültig
}
```

## Borrowing und Lifetimes verstehen

**Borrowing** (Ausleihen) erlaubt es, auf einen Wert zuzugreifen, ohne den Besitzer zu ändern. Dabei unterscheidet Rust zwischen **mutable** und **immutable Borrowing**.

### Borrowing-Regeln:
1. Es kann beliebig viele immutable Borrows geben.
2. Es kann **nur ein** mutable Borrow geben.
3. Immutable und mutable Borrows können nicht gleichzeitig existieren.

### Beispiel für Borrowing:
```rust
fn main() {
    let s = String::from("Hallo");
    let len = calculate_length(&s); // Immutable Borrow

    println!("Die Länge von '{}' ist {}.", s, len);
}

fn calculate_length(s: &String) -> usize {
    s.len() // Greift nur lesend zu
}
```

### Mutable Borrowing:
```rust
fn main() {
    let mut s = String::from("Hallo");

    change(&mut s); // Mutable Borrow

    println!("{}", s); // Ausgabe: Hallo, Welt
}

fn change(s: &mut String) {
    s.push_str(", Welt"); // Ändert den Wert
}
```

### Lifetimes verstehen
Lifetimes geben an, wie lange eine Referenz gültig ist. Sie verhindern **Dangling References**, also Referenzen auf Speicher, der bereits freigegeben wurde.

#### Beispiel für Lifetimes:
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("kurz");
    let string2 = String::from("lang");

    let result = longest(&string1, &string2);
    println!("Das längste Wort ist '{}'", result);
}
```
- `'a` ist eine Lifetime-Annotation, die angibt, dass der Rückgabewert so lange gültig ist wie die kürzere der beiden Eingabe-Referenzen.

### Best Practices mit Borrowing und Lifetimes
1. **Minimale Lifetimes**: Definieren Sie Lifetimes nur, wenn es notwendig ist.
2. **Ownership transferieren**: Übergeben Sie Werte, wenn möglich, anstatt Borrowing zu verwenden.
3. **Referenzen bevorzugen**: Verwenden Sie Referenzen für lesenden Zugriff, um unnötige Kopien zu vermeiden.
