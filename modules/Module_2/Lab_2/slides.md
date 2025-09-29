# Datenstrukturen mit Systemnähe – Theorie & Codebeispiele


**Thema:** Datenstrukturen mit Systemnähe (Lab 2.2)

In den Folien werden komplexere Datentypen vorgestellt. Es gibt ein Beispiel für die Definition einer Struktur `User` mit Feldern `username` (String) und `active` (bool). Eine Instanz dieser Struktur wird erstellt und ausgegeben. Außerdem werden Enums vorgestellt, z. B. `enum Message { Quit, Move { x: i32, y: i32 }, Write(String) }`, und wie sie mit `match` verarbeitet werden.

Für sequenzielle Daten zeigt eine Folie Arrays (`[T; n]`), Slices (`&[T]`) und dynamische Vektoren (`Vec<T>`). Ein Beispiel initialisiert einen Vektor mit `Vec::new()` und fügt mit `push` Elemente hinzu. Schließlich werden assoziative Datenstrukturen wie `HashMap` und `HashSet` demonstriert: ein `HashMap<String, i32>` mit Spielernamen und Punkten, abrufen per `get`, und ein `HashSet` zum Speichern einzigartiger Strings.

**Codebeispiele aus den Folien**
```rust
// Struct
struct User {
    username: String,
    active: bool,
}
let user = User { username: String::from("alice"), active: true };

// Enum
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}
let m = Message::Move { x: 3, y: 4 };
match m {
    Message::Quit => println!("Quit"),
    Message::Move { x, y } => println!("Move {} {}", x, y),
    Message::Write(text) => println!("{}", text),
}

// Arrays, Slices und Vektoren
let arr = [1, 2, 3];
let slice: &[i32] = &arr[0..2];
let mut vec = Vec::new();
vec.push(10);
vec.push(20);

// HashMap und HashSet
use std::collections::{HashMap, HashSet};
let mut scores = HashMap::new();
scores.insert("Alice", 10);
scores.insert("Bob", 20);
if let Some(score) = scores.get("Alice") {
    println!("Alice hat {} Punkte", score);
}
let mut fruits = HashSet::new();
fruits.insert("Apfel");
fruits.insert("Banane");
fruits.insert("Apfel"); // Duplikat wird ignoriert
println!("{} Früchte", fruits.len());
```

