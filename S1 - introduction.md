
# Rust Schulung: Einführung
## Vorstellung von Rust: Geschichte und Ziele

### Geschichte von Rust
Rust wurde 2006 von Graydon Hoare entwickelt und später von Mozilla Research gefördert. Seitdem hat es sich zu einer der beliebtesten Programmiersprachen für Systemprogrammierung und sichere Anwendungen entwickelt. Die Sprache wird heute von der Rust Foundation betreut.

### Ziele von Rust
Rust wurde mit dem Ziel entwickelt, die Lücke zwischen Leistung und Sicherheit zu schließen. Die wichtigsten Ziele von Rust sind:
- **Speichersicherheit**: Rust garantiert durch sein strenges Typsystem und das Borrowing-Modell, dass keine Speicherzugriffsfehler auftreten.
- **Parallelität**: Rust ermöglicht die Entwicklung von sicherem und performantem parallelen Code.
- **Zero-Cost Abstractions**: Die Abstraktionen in Rust bringen keine zusätzlichen Laufzeitkosten mit sich.

Rust wird oft für folgende Anwendungsfälle verwendet:
- Systemnahe Programmierung (z. B. Betriebssysteme)
- WebAssembly
- Netzwerkdienste
- Spieleentwicklung
- Eingebettete Systeme

### Vorteile von Rust
- Kein Garbage Collector: Direkte Kontrolle über den Speicher.
- Hohe Geschwindigkeit, vergleichbar mit C und C++.
- Starke Community und umfangreiche Dokumentation.

## Was ist rustup, rustc und cargo?

### rustup
`rustup` ist ein Tool zur Verwaltung von Rust-Versionen und -Toolchains. Es ermöglicht:
- Die Installation und Aktualisierung von Rust.
- Den Wechsel zwischen verschiedenen Rust-Versionen.
- Die Installation von Zielarchitekturen für Cross-Compilation.

#### Installation von rustup:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Wichtige rustup-Befehle:
- `rustup update`: Aktualisiert Rust auf die neueste Version.
- `rustup show`: Zeigt die installierte Rust-Toolchain an.
- `rustup target add <target>`: Fügt ein Ziel für Cross-Compilation hinzu.

### rustc
`rustc` ist der Compiler von Rust. Es übersetzt Rust-Quellcode in maschinenlesbare Binärdateien.

#### Beispiel:
Erstellen und kompilieren Sie eine Rust-Datei:
1. Erstellen Sie eine Datei `main.rs` mit folgendem Inhalt:
   ```rust
   fn main() {
       println!("Hallo, Welt!");
   }
   ```
2. Kompilieren Sie die Datei mit:
   ```bash
   rustc main.rs
   ```
3. Führen Sie die generierte Binärdatei aus:
   ```bash
   ./main
   ```

### cargo
`cargo` ist das Build-System und Paketmanager für Rust. Es vereinfacht die Verwaltung von Abhängigkeiten, das Erstellen von Projekten und das Testen.

#### Wichtige cargo-Befehle:
- `cargo new <projektname>`: Erstellt ein neues Rust-Projekt.
- `cargo build`: Kompiliert das Projekt.
- `cargo run`: Führt das Projekt aus.
- `cargo test`: Führt Tests aus.
- `cargo doc --open`: Erstellt die Dokumentation für das Projekt und öffnet sie im Browser.

#### Beispiel:
Erstellen und ausführen eines neuen Projekts:
1. Erstellen Sie ein neues Projekt:
   ```bash
   cargo new hallo_welt
   cd hallo_welt
   ```
2. Bearbeiten Sie die Datei `src/main.rs`:
   ```rust
   fn main() {
       println!("Hallo, Welt!");
   }
   ```
3. Bauen und starten Sie das Projekt:
   ```bash
   cargo run
   ```