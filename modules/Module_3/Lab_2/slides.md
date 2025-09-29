# Referenzen & Borrowing verstehen – Theorie & Codebeispiele


**Thema:** Referenzen & Borrowing verstehen (Lab 3.2)

Rust erlaubt neben dem Besitzkonzept auch **ausleihen (Borrowing)**, um ohne Besitzübertragung auf Daten zuzugreifen. Es gibt zwei Arten von Referenzen: **immutable Borrows** (`&T`) und **mutable Borrows** (`&mut T`). Mehrere immutable Referenzen sind gleichzeitig erlaubt, aber während eine mutable Referenz existiert, darf keine andere Referenz existieren. Die Folien enthalten ein Beispiel, in dem zwei immutable Referenzen `r1` und `r2` auf einen String erlaubt sind, aber das Erstellen einer zusätzlichen mutable Referenz `r3` im gleichen Scope verboten ist. Durch das Beenden des Gültigkeitsbereichs der immutablen Referenzen kann später eine mutable Referenz erstellt werden.

**Codebeispiele aus den Folien**
```rust
fn main() {
    let mut s = String::from("Hallo");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);
    // let r3 = &mut s; // nicht erlaubt: r1 und r2 sind noch im Scope
    // r3.push_str(" Rust");
}

// Null‑Safety mit Option
fn find_item(x: i32) -> Option<i32> {
    if x > 0 { Some(x * 2) } else { None }
}
```

