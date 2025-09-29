# Lifetimes einfach erklärt – Lösungen

## Aufgabe 1

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
fn main() {
    let a = String::from("Hallo");
    let b = String::from("Welt!");
    println!("Länger: {}", longest(&a, &b));
}
```
**Erläuterung:** Die Lifetime‑Annotation `'a` sagt dem Compiler, dass der Rückgabewert dieselbe Lebensdauer hat wie die kürzeste der beiden Eingaben.


## Aufgabe 2

```rust
fn first_word<'a>(s: &'a str) -> &'a str {
    for (i, c) in s.char_indices() {
        if c == ' ' {
            return &s[..i];
        }
    }
    s
}
fn main() {
    let phrase = "Hallo Rust Welt";
    let word = first_word(phrase);
    println!("Erstes Wort: {}", word);
}
```
**Erläuterung:** Der Rückgabewert ist ein Slice der Eingabezeichenkette. Lifetime‑Annotationen stellen sicher, dass der Rückgabeslice nicht länger lebt als die ursprüngliche Zeichenkette. Ohne Annotationen könnte der Compiler nicht ableiten, wie die Lebensdauern zusammenhängen.


## Aufgabe 3

```rust
struct Excerpt<'a> {
    text: &'a str,
}
impl<'a> Excerpt<'a> {
    fn level(&self) -> i32 { 1 }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("{}", announcement);
        self.text
    }
}
fn main() {
    let novel = String::from("Es war einmal ein König in einem fernen Land.");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = Excerpt { text: first_sentence };
    println!("Level: {}", excerpt.level());
    println!("Auszug: {}", excerpt.announce_and_return_part("Hier ist mein Auszug:"));
}
```
**Erläuterung:** Die Struktur speichert einen Slice auf eine `String`. Die Lifetime `'a` stellt sicher, dass `Excerpt` nicht länger lebt als die zugrundeliegende `String`. Methoden, die Slices zurückgeben, benötigen ebenfalls diese Lifetime, um gültige Referenzen zu garantieren.


