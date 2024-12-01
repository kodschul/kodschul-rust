
# Rust Schulung: Best Practices, Makros, Dokumentation, Unittesting

## Best Practices

### 1. Speicher- und Sicherheitsbewusstsein
Rusts Ownership-Modell sorgt für Speicher- und Sicherheitsgarantien ohne Garbage Collection. Befolgen Sie diese Regeln:
- Verwenden Sie `&` und `&mut` für Referenzen anstelle von Besitzübergaben, wenn möglich.
- Vermeiden Sie unsicheren Code (`unsafe`), außer wenn es unvermeidbar ist.

### 2. Effiziente Nutzung von Enums und Pattern Matching
Enums und Pattern Matching sind leistungsstarke Werkzeuge für sauberen Code.
```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn move_character(direction: Direction) {
    match direction {
        Direction::Up => println!("Moving up"),
        Direction::Down => println!("Moving down"),
        Direction::Left => println!("Moving left"),
        Direction::Right => println!("Moving right"),
    }
}
```

### 3. Verwendung von Result und Option
Nutzen Sie `Result` und `Option`, um Fehler sicher zu handhaben.
```rust
fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Division durch Null")
    } else {
        Ok(a / b)
    }
}
```

### 4. Modularisierung und Crates
Teilen Sie Ihren Code in kleinere, gut definierte Module oder Crates auf, um die Wartbarkeit zu verbessern.

---

## Makros

Makros in Rust sind mächtige Werkzeuge zur Codegenerierung und ermöglichen wiederverwendbaren, flexiblen Code.

### Deklarative Makros
Deklarative Makros verwenden die Syntax `macro_rules!`, um Muster zu definieren.
```rust
macro_rules! say_hello {
    () => {
        println!("Hallo, Welt!");
    };
}

fn main() {
    say_hello!();
}
```

### Prozedurale Makros
Prozedurale Makros arbeiten auf Code als Token-Streams und bieten mehr Flexibilität.
- **Attribute Makros**: Verändern Funktionen oder Strukturen.
- **Derive Makros**: Automatisieren die Implementierung von Traits.
- **Function-like Makros**: Funktionieren ähnlich wie Deklarative Makros, haben aber Zugriff auf mehr Compiler-Interna.

Beispiel für ein benutzerdefiniertes `derive` Makro:
```rust
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}
```

---

## Dokumentation

Rust verwendet `rustdoc`, um aus Kommentaren und Code direkt HTML-Dokumentation zu generieren.

### Inline-Dokumentation
Fügen Sie Dokumentationskommentare mit `///` hinzu.
```rust
/// Berechnet die Summe zweier Zahlen.
///
/// # Beispiele
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### Modul-Dokumentation
Fügen Sie Dokumentationen zu Modulen mit `//!` hinzu.
```rust
//! Dieses Modul enthält grundlegende mathematische Operationen.
pub mod math {
    /// Addiert zwei Zahlen.
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}
```

### Generieren der Dokumentation
Verwenden Sie `cargo doc`:
```bash
cargo doc --open
```

---

## Unittesting

Rust bietet integrierte Unterstützung für Unit-Tests.

### Beispiel für einen Unit-Test
```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    #[should_panic]
    fn test_add_panic() {
        panic!("Dieser Test sollte fehlschlagen!");
    }
}
```

### Integrationstests
Integrationstests befinden sich in einem separaten Ordner `tests`.
```rust
// Datei: tests/integration_test.rs
use my_crate::add;

#[test]
fn integration_test_add() {
    assert_eq!(add(10, 5), 15);
}
```

### Testausführung
Führen Sie Tests mit `cargo test` aus:
```bash
cargo test
```

---