# Lifetimes einfach erklärt – Aufgaben

## Aufgabe 1
Schreibe eine generische Funktion `longest<'a>(x: &'a str, y: &'a str) -> &'a str`, die die längere der beiden Zeichenketten zurückgibt. Teste sie mit zwei Strings.

## Aufgabe 2
Implementiere die Funktion `first_word` wie im Beispiel und erkläre, warum eine Lifetime‑Annotation notwendig ist.

## Aufgabe 3
Definiere eine Struktur `Excerpt<'a>` mit einem Feld `text: &'a str`. Implementiere eine Methode `level(&self) -> i32`, die eine feste Zahl zurückgibt, und eine Methode `announce_and_return_part(&self, announcement: &str) -> &str`, die zuerst die Ankündigung ausgibt und dann den gespeicherten Text zurückgibt. Erkläre, warum Lifetimes erforderlich sind.

