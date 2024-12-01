
# Rust Schulung: Modulverwaltung
## Packages und Crates

In Rust sind **Packages** und **Crates** die grundlegenden Bausteine für die Organisation von Code.

### Packages
- Ein **Package** ist eine Sammlung von Rust-Projekten, die aus einem oder mehreren Crates bestehen können.
- Jedes Package enthält standardmäßig eine `Cargo.toml`-Datei, die die Abhängigkeiten und Metadaten des Projekts definiert.
- Ein Package kann nur **ein** Bibliothekscrate haben, aber beliebig viele Binärcrates.

Beispiel: Inhalt der `Cargo.toml`
```toml
[package]
name = "my_package"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"
```

### Crates
- Ein **Crate** ist eine kompakte Einheit, die vom Rust-Compiler verarbeitet wird.
- Es gibt zwei Arten von Crates:
  - **Binary Crates**: Enthalten einen ausführbaren Einstiegspunkt (die `main`-Funktion).
  - **Library Crates**: Exportieren Funktionen, Typen oder Module, die von anderen Crates genutzt werden können.

Beispiel: Einfache `main.rs` für ein Binary Crate
```rust
fn main() {
    println!("Hallo, Rust!");
}
```

## Module

**Module** ermöglichen es, Code in logische Einheiten zu organisieren. Sie helfen, Namenskonflikte zu vermeiden und den Code besser lesbar zu machen.

### Deklaration von Modulen
Module können inline oder in separaten Dateien definiert werden.

#### Inline-Definition
```rust
mod greetings {
    pub fn say_hello() {
        println!("Hallo, Welt!");
    }
}

fn main() {
    greetings::say_hello();
}
```

#### In separaten Dateien
- Legen Sie eine Datei `greetings.rs` im gleichen Verzeichnis wie `main.rs` an.
- Deklarieren Sie das Modul in `main.rs`:
```rust
mod greetings;

fn main() {
    greetings::say_hello();
}
```

Datei `greetings.rs`:
```rust
pub fn say_hello() {
    println!("Hallo, Welt!");
}
```

## Zugriffskontrolle

Rust verwendet Schlüsselwörter zur Kontrolle des Zugriffs auf Module, Funktionen und Variablen.

- **`pub`**: Macht einen Bestandteil öffentlich und zugänglich.
- Standardmäßig ist alles in Rust **privat**.

Beispiel:
```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
}

fn main() {
    println!("{}", math::add(5, 3)); // Funktion ist öffentlich
    // println!("{}", math::subtract(5, 3)); // Fehler: Funktion ist privat
}
```

## Pfad-Syntax

Die Pfad-Syntax wird verwendet, um auf Module, Funktionen oder Typen zuzugreifen.

### Absoluter Pfad
Beginnt immer mit dem Namen des Crates.
```rust
crate::module_name::function_name();
```

### Relativer Pfad
Verwendet `self`, `super` oder den Namen des Moduls, um auf Bestandteile innerhalb der aktuellen Hierarchie zuzugreifen.

Beispiel:
```rust
mod parent {
    pub mod child {
        pub fn say_hello() {
            println!("Hallo aus dem Kind-Modul!");
        }
    }
}

fn main() {
    // Absoluter Pfad
    crate::parent::child::say_hello();

    // Relativer Pfad
    parent::child::say_hello();
}
```

### `use`-Anweisung
Verkürzt den Zugriff auf Module oder Funktionen:
```rust
mod greetings {
    pub fn say_hello() {
        println!("Hallo!");
    }
}

use greetings::say_hello;

fn main() {
    say_hello();
}
```