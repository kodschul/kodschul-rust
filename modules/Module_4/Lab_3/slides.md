# Best Practices für den Alltag – Theorie & Codebeispiele


**Thema:** Best Practices für den Alltag (Lab 4.3)

Dieses Lab fasst bewährte Praktiken für idiomatischen Rust‑Code zusammen. Die Folien warnen vor unnötigen Aufrufen von `.clone()` oder `.unwrap()`, da diese oft Fehler verbergen oder zu unnötigen Kopien führen. Stattdessen wird empfohlen, mit Mustern wie `if let Ok(val) = result` oder dem `?`‑Operator zu arbeiten.

Außerdem wird gezeigt, wie man Panics in Bibliothekscode vermeidet und stattdessen Fehler mit `Result<T, E>` nach außen gibt. Eine Folie illustriert eine idiomatische Projektstruktur: `src/main.rs` enthält die Einstiegspunkte, Logik liegt in `src/lib.rs`, und separate Module, Tests und Beispiele befinden sich in eigenen Unterordnern.

**Codebeispiele aus den Folien**
```rust
// Unidiomatischer Code
fn might_fail() -> Result<i32, &str> { Ok(42) }
fn main() {
    let result = might_fail();
    if result.is_ok() {
        println!("{}", result.unwrap());
    }
}
// Idiomatischer Code
fn main() {
    if let Ok(val) = might_fail() {
        println!("{}", val);
    }
}
```
Eine Abbildung zeigt eine typische Projektstruktur:
```
my_project/
├─ Cargo.toml
├─ src/
│  ├─ main.rs
│  ├─ lib.rs
│  └─ utils.rs
├─ tests/
├─ benches/
├─ examples/
```

