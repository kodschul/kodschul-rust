# Funktionen, Traits und Generics – Aufgaben

## Aufgabe 1
Schreibe eine Funktion `square`, die ein `i32` entgegennimmt und sein Quadrat zurückgibt. Rufe die Funktion in `main` mit mehreren Werten auf.

## Aufgabe 2
Definiere einen Trait `Printable` mit der Methode `print(&self)`. Implementiere ihn für `i32` und `String`, sodass die Werte mit `println!` ausgegeben werden. Schreibe eine Funktion, die ein generisches Argument akzeptiert, das `Printable` implementiert, und rufe diese Funktion mit verschiedenen Typen auf.

## Aufgabe 3
Implementiere eine generische Funktion `min<T: PartialOrd>(list: &[T]) -> &T`, die das kleinste Element in einer Liste zurückgibt. Teste sie mit Vektoren von Ganzzahlen und Zeichen.

