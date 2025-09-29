# Rust in der Praxis: Was kann ersetzt werden? – Aufgaben

## Aufgabe 1
Stelle dir vor, du hast in C eine Funktion geschrieben, die ein Array manuell verwaltet und dessen Größe selbst verfolgt. Ersetze diese Logik in Rust durch einen `Vec<i32>` und schreibe eine Funktion `average`, die den Durchschnitt der Werte berechnet.

## Aufgabe 2
Implementiere ein sicheres Pendant zu einer C‑Funktion, die einen rohen Zeiger erhält und ein Element am Index zurückgibt. Verwende in Rust einen Slice‑Parameter `&[u8]` und gib `Option<u8>` zurück, falls der Index außerhalb des Bereichs liegt.

## Aufgabe 3
Diskutiere, welche Arten von C/C++‑Code du in zukünftigen Projekten bevorzugt durch Rust ersetzen würdest. Nenne konkrete Gründe wie Speicher‑ und Thread‑Sicherheit.

