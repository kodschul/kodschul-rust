# Best Practices für den Alltag – Aufgaben

## Aufgabe 1
Gegeben ist eine Funktion `might_fail() -> Result<i32, &str>`. Schreibe `main` einmal unidiomatisch mit `is_ok()` und `unwrap()`, und anschließend idiomatisch mit `if let` oder dem `?`‑Operator.

## Aufgabe 2
Vermeide unnötiges Kopieren: Schreibe eine Funktion, die die Summe eines Vektors berechnet, ohne `clone()` zu verwenden. Erkläre, warum ein `clone()` hier überflüssig ist.

## Aufgabe 3
Skizziere eine typische Projektstruktur für ein Rust‑Projekt mit `main.rs`, `lib.rs`, Modulen, Tests und Beispielen. Erstelle diese Verzeichnisse und Dateien in einem neuen Projekt.

