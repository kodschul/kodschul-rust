
# Rust Schulung: Die Grundlagen
## Variablen und Datentypen

### 1. Variablen
In Rust sind Variablen standardmäßig **immutable** (unveränderlich). Um eine veränderliche Variable zu erstellen, muss das Schlüsselwort `mut` verwendet werden.

Beispiel:
```rust
fn main() {
    let x = 5; // Unveränderlich
    println!("x: {}", x);

    let mut y = 10; // Veränderlich
    println!("y: {}", y);
    y = 15; // Wert ändern
    println!("y: {}", y);
}
```

### 2. Shadowing
Mit Shadowing können Variablen mit demselben Namen erneut deklariert werden, wobei der vorherige Wert überschrieben wird.
```rust
fn main() {
    let x = 5;
    let x = x + 1; // Shadowing
    println!("x: {}", x);
}
```

### 3. Datentypen
Rust ist eine **statically typed** Sprache, d.h., der Typ jeder Variablen wird entweder explizit oder implizit bestimmt.

- **Integer**: `i32`, `u32` (mit und ohne Vorzeichen)
- **Floating Point**: `f32`, `f64`
- **Boolean**: `bool`
- **Character**: `char`
- **String**: `String` (Heap-zugeordnet) und `&str` (String-Slices)

Beispiel:
```rust
fn main() {
    let int: i32 = 42;
    let float: f64 = 3.14;
    let boolean: bool = true;
    let character: char = 'R';
    let string: String = String::from("Rust");

    println!("{}, {}, {}, {}, {}", int, float, boolean, character, string);
}
```

## Funktionen und Kontrollstrukturen

### 1. Funktionen
Funktionen sind zentrale Bausteine in Rust. Sie werden mit `fn` definiert.

Beispiel:
```rust
fn main() {
    greet("Rust");
    let result = add(5, 3);
    println!("Result: {}", result);
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b // Letzte Zeile ohne Semikolon ist der Rückgabewert
}
```

### 2. Kontrollstrukturen

#### **if-else**
Rust unterstützt die klassische if-else-Syntax. Bedingungen müssen immer vom Typ `bool` sein.
```rust
fn main() {
    let number = 7;

    if number < 10 {
        println!("Kleinere Zahl");
    } else {
        println!("Größere Zahl");
    }
}
```

#### **Loops**
Rust bietet drei Arten von Schleifen:
- `loop`: Endlos-Schleifen
- `while`: Schleifen mit Bedingung
- `for`: Iteration über eine Range oder ein Collection

Beispiele:
```rust
fn main() {
    // Endlos-Schleife
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            break;
        }
    }

    // While-Schleife
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    // For-Schleife
    for i in 1..4 {
        println!("i: {}", i);
    }
}
```

### 3. Match
`match` ist eine leistungsstarke Kontrollstruktur, die Musterabgleich unterstützt.
```rust
fn main() {
    let number = 2;

    match number {
        1 => println!("Eins"),
        2 => println!("Zwei"),
        3 => println!("Drei"),
        _ => println!("Etwas anderes"), // _ ist der Standardfall
    }
}
```