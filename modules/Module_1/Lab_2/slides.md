# Performance wie C – aber sicher – Theorie & Codebeispiele


**Thema:** Performance wie C – aber sicher (Lab 2)

Dieses Lab vergleicht niedrige und hohe Abstraktionsebenen in Rust. Eine Folie zeigt zwei Varianten, um Werte in einem Vektor zu verdoppeln und zu summieren: einmal mithilfe von Iteratoren (`map`/`sum`) und einmal mit einer expliziten `for`‑Schleife. Beide Ansätze sind gleich effizient, aber die Iterator‑Version ist kompakter und ausdrucksstärker.

Außerdem wird die Kontrollflussstruktur `match` vorgestellt, die sicherstellt, dass alle Fälle abgedeckt werden. Ein Beispiel wandelt einen numerischen Code in eine Zeichenkette um und verwendet `match` mit einem Default‑Fall (`_`). Weitere Folien zeigen unterschiedliche Schleifen: eine endlose `loop`, eine `while`‑Schleife und eine `for`‑Schleife.

**Codebeispiele aus den Folien**
```rust
// Iterator vs. Schleife
let nums = vec![1, 2, 3, 4];
let sum_iter: i32 = nums.iter().map(|x| x * 2).sum();
let mut sum_loop = 0;
for x in &nums {
    sum_loop += x * 2;
}

// match zur Dekodierung
fn decode(code: u8) -> &'static str {
    match code {
        1 => "Start",
        2 => "Stop",
        _ => "Unbekannt",
    }
}

// loop/while/for
let mut i = 0;
loop {
    if i >= 3 { break; }
    i += 1;
}
let mut j = 0;
while j < 3 {
    j += 1;
}
for k in 0..3 {
    println!("{}", k);
}
```

