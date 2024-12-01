
# Rust Schulung: Fehlerbehandlung
## Nicht wieder herstellbare Fehler mit `panic!`

### Was ist `panic!`?
Der Makro `panic!` wird verwendet, wenn ein schwerwiegender Fehler auftritt, der nicht wiederhergestellt werden kann. Das Programm wird sofort beendet, und der Call-Stack wird ausgegeben, um den Fehler zu debuggen.

#### Beispiel:
```rust
fn main() {
    let numbers = vec![1, 2, 3];
    println!("{}", numbers[10]); // Zugriff außerhalb des Bereichs löst panic! aus
}
```

### Verwendung von `panic!`
Das Makro `panic!` kann auch explizit verwendet werden:
```rust
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division durch Null ist nicht erlaubt!");
    }
    a / b
}

fn main() {
    divide(10, 0); // Löst panic! aus
}
```

### Best Practices für `panic!`
- Verwenden Sie `panic!` nur in Situationen, in denen der Fehler so schwerwiegend ist, dass das Programm nicht sinnvoll fortgesetzt werden kann.
- Für reguläre Fehler verwenden Sie den `Result`-Typ.

---

## Handling von Fehlern mit `Result`

### Was ist `Result`?
Der Typ `Result<T, E>` wird verwendet, um wieder herstellbare Fehler zu behandeln. Er hat zwei Varianten:
- `Ok(T)` für den Erfolg
- `Err(E)` für den Fehler

#### Beispiel:
```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division durch Null ist nicht erlaubt!"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10, 0) {
        Ok(result) => println!("Ergebnis: {}", result),
        Err(e) => println!("Fehler: {}", e),
    }
}
```

### Fehler-Handling mit `match`
Das Muster-Matching mit `match` ist der häufigste Weg, um den `Result`-Typ zu verarbeiten.

#### Beispiel:
```rust
fn open_file(filename: &str) -> Result<(), std::io::Error> {
    std::fs::File::open(filename)?;
    Ok(())
}

fn main() {
    match open_file("nonexistent.txt") {
        Ok(_) => println!("Datei erfolgreich geöffnet."),
        Err(e) => println!("Fehler: {:?}", e),
    }
}
```

### Der `?`-Operator
Der `?`-Operator ist eine Kurzschreibweise für die Verarbeitung von `Result`. Er gibt den Fehler frühzeitig zurück, falls einer auftritt.

#### Beispiel:
```rust
use std::fs::File;

fn read_file() -> Result<String, std::io::Error> {
    let mut file = File::open("example.txt")?;
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents)?;
    Ok(contents)
}
```

---

## Unterschiede zwischen `panic!` und `Result`

| **Eigenschaft**               | **panic!**                     | **Result**                    |
|-------------------------------|---------------------------------|--------------------------------|
| Anwendungsfall               | Nicht wieder herstellbare Fehler | Wieder herstellbare Fehler    |
| Programmabbruch              | Ja                              | Nein                          |
| Rückgabewert                 | Kein Rückgabewert               | Erfolg oder Fehler            |
| Verwendung in Produktion     | Vermeiden                      | Empfohlen                     |

---