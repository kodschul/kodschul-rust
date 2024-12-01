
# Rust Schulung: Structs und Enums
## Erstellen von Strukturen und Enums

### 1. Strukturen (Structs)
Strukturen in Rust werden verwendet, um benutzerdefinierte Datentypen zu erstellen, die mehrere Felder enthalten können.

#### Beispiel: Eine einfache Struktur
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Die Fläche des Rechtecks ist {} Quadratpixel.", area(&rect));
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
```

#### Arten von Structs
- **Klassische Structs**:
  ```rust
  struct Point {
      x: i32,
      y: i32,
  }
  ```
- **Tuple Structs**: Felder werden durch Position referenziert.
  ```rust
  struct Color(i32, i32, i32);
  ```
- **Unit-like Structs**: Keine Felder, häufig für Marker verwendet.
  ```rust
  struct Marker;
  ```

### 2. Enums
Enums in Rust ermöglichen es, einen Typ zu definieren, der mehrere verschiedene Werte annehmen kann.

#### Beispiel: Einfache Enum
```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {
    let light = TrafficLight::Red;
    println!("Die Ampel zeigt: {:?}", light);
}
```

#### Enum mit Daten
Enums können auch zusätzliche Daten speichern:
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::Move { x: 10, y: 20 };
    match msg {
        Message::Move { x, y } => println!("Bewegung zu x: {}, y: {}", x, y),
        _ => (),
    }
}
```

## Pattern Matching mit Enums

Pattern Matching ist ein mächtiges Werkzeug in Rust, um den Wert eines Enums zu prüfen und darauf zu reagieren.

### 1. Verwendung von `match`
`match` ist eine Kontrollflussstruktur, die alle möglichen Varianten eines Enums prüft.

#### Beispiel:
```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin = Coin::Dime;
    println!("Der Wert der Münze ist {} Cent.", value_in_cents(coin));
}
```

### 2. Nutzung von `if let`
Für Fälle, bei denen nur eine Variante geprüft werden muss, kann `if let` verwendet werden.

#### Beispiel:
```rust
enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let some_value = Option::Some(5);

    if let Option::Some(val) = some_value {
        println!("Der Wert ist: {}", val);
    } else {
        println!("Kein Wert vorhanden.");
    }
}
```