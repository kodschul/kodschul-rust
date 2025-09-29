# Sicherheit ohne Garbage Collector – Theorie & Codebeispiele


**Thema:** Sicherheit ohne Garbage Collector (Lab 1)

In diesem Lab wird das Speichermanagement von Rust ohne Garbage Collector demonstriert. Rust verfolgt das **Ownership Modell**: jedes Objekt hat genau einen Besitzer und wird automatisch am Ende des Gültigkeitsbereichs freigegeben. Dadurch ist die Speicherverwaltung zur Compile‑Zeit überprüfbar und es entsteht kein Overhead durch eine Laufzeit‑GC. Die Folien zeigen ein einfaches Beispiel, in dem eine `String` erstellt, ausgegeben und am Ende des `main`‑Scopes automatisch freigegeben wird. Der Compiler garantiert, dass auf die freigegebenen Daten nicht mehr zugegriffen werden kann【674319147427555†L140-L149】.

Weitere Folien veranschaulichen das **Move‑Semantik**: beim Zuweisen einer Variable an eine andere wird der Besitz verschoben. Der ursprüngliche Name kann danach nicht mehr verwendet werden, was potentielle Use‑After‑Free‑Fehler verhindert. Dies wird mit einem `String` verdeutlicht, der an eine zweite Variable übergeben wird und danach nicht mehr nutzbar ist.

Zusätzlich wird der Umgang mit optionalen Werten über `Option<T>` gezeigt. Anstatt auf `null` zu vertrauen, liefert eine Funktion `Option` zurück. Im gezeigten Beispiel berechnet `find_item` das Doppelte nur für positive Zahlen und gibt `None` zurück, falls das Argument negativ ist; der Aufrufer muss beide Fälle behandeln【674319147427555†L221-L229】.

**Codebeispiele aus den Folien**
```rust
// RAII: Eine String wird am Ende des Scopes freigegeben
fn main() {
    let s = String::from("Hallo Rust");
    println!("{}", s);
    // s wird hier automatisch gelöscht (drop)
}

// Move‑Semantik: b übernimmt den Besitz von a
let a = String::from("Test");
let b = a; // a ist danach ungültig
// println!("{}", a); // Fehler: a wurde verschoben

// Option‑Beispiel
fn find_item(x: i32) -> Option<i32> {
    if x > 0 { Some(x * 2) } else { None }
}
fn main() {
    let result = find_item(3);
    match result {
        Some(val) => println!("Wert: {}", val),
        None => println!("Kein Ergebnis"),
    }
}
```

