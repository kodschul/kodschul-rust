
# Rust Schulung: Generics und Traits
## Einführung in Generics

Generics erlauben es, Code so zu schreiben, dass er mit verschiedenen Typen funktioniert, ohne diese Typen explizit festzulegen. Dies verbessert die Wiederverwendbarkeit und Flexibilität des Codes.

### Generics in Funktionen

Eine generische Funktion kann mit beliebigen Typen arbeiten, solange diese Typen die Anforderungen erfüllen.

#### Beispiel:
```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers = vec![10, 20, 5, 7];
    println!("Das größte Element ist: {}", largest(&numbers));
}
```

- **`<T>`**: Der Platzhalter für den generischen Typ.
- **`PartialOrd`**: Ein Trait, der sicherstellt, dass die Elemente vergleichbar sind.

### Generics in Structs

Structs können ebenfalls generische Typen verwenden:
```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
}
```

### Generics in Enums

Enums profitieren ebenfalls von Generics:
```rust
enum Option<T> {
    Some(T),
    None,
}
```

### Vorteile von Generics
- **Wiederverwendbarkeit**: Einmal geschrieben, können generische Funktionen oder Datentypen für verschiedene Typen verwendet werden.
- **Typensicherheit**: Rust stellt sicher, dass der generische Code nur mit kompatiblen Typen verwendet wird.

## Verwendung von Traits

Traits sind eine Sammlung von Methoden, die eine bestimmte Funktionalität definieren. Sie sind ähnlich wie Schnittstellen in anderen Programmiersprachen.

### Definition eines Traits

Ein Trait wird mit dem Schlüsselwort `trait` definiert:
```rust
trait Summary {
    fn summarize(&self) -> String;
}
```

### Implementierung eines Traits

Typen implementieren Traits durch das Schlüsselwort `impl`:
```rust
struct NewsArticle {
    headline: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}: {}", self.headline, self.content)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Rust 1.67 Released!"),
        content: String::from("Exciting new features in Rust."),
    };
    println!("{}", article.summarize());
}
```

### Traits als Parameter

Traits können verwendet werden, um den Typ von Funktionsparametern einzuschränken:
```rust
fn notify(item: &impl Summary) {
    println!("Nachricht: {}", item.summarize());
}
```

### Trait Bounds

Mit Trait Bounds können generische Funktionen auf Typen beschränkt werden, die einen bestimmten Trait implementieren:
```rust
fn notify<T: Summary>(item: &T) {
    println!("Nachricht: {}", item.summarize());
}
```

### Standardmethoden in Traits

Traits können Standardimplementierungen für ihre Methoden enthalten:
```rust
trait Summary {
    fn summarize(&self) -> String {
        String::from("(Zusammenfassung nicht verfügbar)")
    }
}
```

### Dynamische Dispatch mit Traits

Traits können auch dynamisch verwendet werden:
```rust
fn notify(item: &dyn Summary) {
    println!("Nachricht: {}", item.summarize());
}
```