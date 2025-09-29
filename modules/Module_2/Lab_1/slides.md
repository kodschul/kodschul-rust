# Skalare Typen, Variablen & Kontrollfluss – Theorie & Codebeispiele


**Thema:** Skalare Typen, Variablen & Kontrollfluss (Lab 2.1)

Die Folien führen die grundlegenden Datentypen von Rust ein: ganzzahlige Typen (`i8`, `i32`, `u64`, …), Fließkommazahlen (`f32`, `f64`), Booleans (`bool`) und Zeichentyp (`char`). Variablen sind standardmäßig unveränderlich und werden mit `let` definiert; mit `mut` können sie veränderlich gemacht werden. Beispiele zeigen das Deklarieren verschiedener Typen und ihre Ausgabe mit `println!`.

Für den Kontrollfluss wird `if`/`else` als Ausdruck gezeigt und die mächtige `match`‑Anweisung, mit der alle möglichen Fälle eines Werts behandelt werden müssen. Außerdem werden die Schleifenkonstrukte `loop`, `while` und `for` demonstriert.

**Codebeispiele aus den Folien**
```rust
// Deklaration verschiedener Typen
let x: i32 = 42;
let pi: f64 = 3.1415;
let flag: bool = true;
let c: char = 'R';
println!("x={}, pi={}, flag={}, c={}", x, pi, flag, c);

// match-Beispiel
let code = 2;
match code {
    1 => println!("Start"),
    2 => println!("Stop"),
    _ => println!("Unbekannt"),
}
```

