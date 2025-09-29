# Rust in der Praxis: Was kann ersetzt werden? – Theorie & Codebeispiele


**Thema:** Rust in der Praxis – Was kann ersetzt werden? (Lab 3)

Diese Folien diskutieren, welche C/C++‑Module durch Rust ersetzt werden können. Rust eignet sich besonders für sicherheitskritische Komponenten, Treiber und systemnahe Bibliotheken, weil es Speichersicherheit ohne Garbage Collector bietet und viele Fehler (z. B. Pufferüberläufe) bereits zur Compile‑Zeit verhindert werden. Auch Legacy‑Code kann schrittweise migriert werden: Rust‑Module können per FFI mit bestehendem C/C++‑Code interagieren und einzelne Komponenten ersetzen. Obwohl in diesem Abschnitt keine konkreten Code‑Snippets gezeigt werden, wird empfohlen, Rust für neue Komponenten einzusetzen und alte C‑Funktionen schrittweise in sichere Rust‑Implementierungen zu überführen.

**Kein spezifisches Codebeispiel auf den Folien**, aber die Konzepte lassen sich auf einfache Beispiele anwenden, um C‑typische Probleme (manuelles Speicher‑Management, undefined behavior) zu vermeiden.

