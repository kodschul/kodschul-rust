# Lifetimes einfach erklärt – Theorie & Codebeispiele


**Thema:** Lifetimes einfach erklärt (Lab 3.3)

Lifetimes stellen sicher, dass Referenzen immer länger leben als die Daten, auf die sie zeigen. Die Folien erklären, dass Funktionen Parameter mit verschiedenen Lebensdauern haben können und wie man mit Lifetime‑Annotationen die Beziehungen zwischen ihnen beschreibt. Ein Beispiel `choose<'a, 'b>(x: &'a str, y: &'b str) -> &'a str` gibt immer `x` zurück und zeigt, dass die Rückgabe mindestens so lange leben muss wie `x`.

Ein weiteres Beispiel ist die Funktion `first_word<'a>(s: &'a str) -> &'a str`, die einen Slice auf das erste Wort einer Zeichenkette zurückgibt. Da der Slice auf die ursprüngliche Zeichenkette verweist, ist dieselbe Lifetime `'a` erforderlich.

**Codebeispiele aus den Folien**
```rust
// choose-Funktion
fn choose<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}
fn main() {
    let s1 = String::from("eins");
    let s2 = String::from("zwei");
    let result = choose(&s1, &s2);
    println!("{}", result);
}

// first_word
fn first_word<'a>(s: &'a str) -> &'a str {
    for (i, c) in s.char_indices() {
        if c == ' ' {
            return &s[..i];
        }
    }
    s
}
```

