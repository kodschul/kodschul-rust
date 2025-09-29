# Funktionen, Traits und Generics – Theorie & Codebeispiele


**Thema:** Funktionen, Traits und Generics (Lab 2.3)

Die Folien zeigen, wie man Funktionen definiert (`fn name(params) -> Rückgabetyp`) und wie man sie aufruft. Ein Beispiel addiert zwei `i32`‑Werte und eine andere Funktion gibt eine Begrüßung zurück.

Außerdem wird das Konzept der Traits eingeführt, mit dem gemeinsame Schnittstellen für verschiedene Typen definiert werden. Eine Folie definiert den Trait `Drawable` mit einer Methode `draw`, implementiert ihn für `Circle` und `Square` und zeigt eine Funktion `render`, die ein generisches Typparameter verwendet, das den Trait implementiert【521177031964362†L142-L145】.

Generics ermöglichen es, Funktionen und Datentypen für beliebige Typen zu formulieren. Die Folien zeigen das generische Beispiel `fn largest<T: PartialOrd + Copy>(list: &[T]) -> T`, das das größte Element in einer Liste zurückgibt【417177216926898†L220-L229】. Ein weiteres Beispiel verwendet das Trait `Summary` und eine generische Funktion `notify<T: Summary>`, die für alle Typen funktioniert, die das Trait implementieren【521177031964362†L170-L190】.

**Codebeispiele aus den Folien**
```rust
// Funktionsbeispiel
fn add(a: i32, b: i32) -> i32 { a + b }
fn greet(name: &str) -> String {
    format!("Hallo, {}!", name)
}

// Trait und Implementierung
trait Drawable {
    fn draw(&self);
}
struct Circle { radius: f32 }
struct Square { side: f32 }
impl Drawable for Circle {
    fn draw(&self) { println!("Kreis mit Radius {}", self.radius); }
}
impl Drawable for Square {
    fn draw(&self) { println!("Quadrat mit Seite {}", self.side); }
}
fn render<T: Drawable>(shape: T) {
    shape.draw();
}

// Generische Funktion
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest { largest = item; }
    }
    largest
}

// Trait + Generics
trait Summary {
    fn summarize(&self) -> String;
}
struct Article { title: String, content: String }
impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}...", &self.title)
    }
}
fn notify<T: Summary>(item: T) {
    println!("Neue Nachricht: {}", item.summarize());
}
```

