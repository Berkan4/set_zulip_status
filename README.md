# Zulip Status Setter

A minimal desktop app to set your Zulip status with a single click. Built with Rust and egui — launches instantly, closes itself automatically after 2 seconds.

Ideal for autostart at PC boot: one click and you're done.

---

## Voraussetzungen / Requirements

- [Rust](https://rustup.rs/) must be installed
- A Zulip account with API access
- Your `.zuliprc` config file (see below)

---

## Anleitung: zuliprc herunterladen

Die `.zuliprc`-Datei enthält deine Zugangsdaten für die Zulip-API. So bekommst du sie:

1. Öffne Zulip und klicke oben rechts auf dein **Profilbild**
2. Wähle **Einstellungen**
3. Gehe zu **Account und Privatsphäre**
4. Scrolle ganz nach unten zum Abschnitt **„Deine API-Schlüssel verwalten"**
5. Gib dein Passwort ein und klicke auf **„zuliprc herunterladen"**
6. Lege die heruntergeladene Datei in folgenden Pfad:

```
C:\Users\DEINNAME\.zuliprc\zuliprc       ← Windows
/home/DEINNAME/.zuliprc/zuliprc          ← Linux
/Users/DEINNAME/.zuliprc/zuliprc         ← macOS
```

> ⚠️ Der Ordner `.zuliprc` muss manuell erstellt werden, falls er noch nicht existiert. Die Datei heißt einfach `zuliprc` (ohne Dateiendung).

---

## Build & Run

```bash
# Debug (schneller Build)
cargo run

# Release (optimierte Binärdatei)
cargo build --release
```

Die fertige `.exe` / Binary liegt dann unter:

```
target/release/zulip_status.exe    ← Windows
target/release/zulip_status        ← Linux / macOS
```

---

## Autostart (Windows)

Um die App beim PC-Start automatisch zu starten:

1. Drücke `Win + R`, gib `shell:startup` ein und drücke Enter
2. Lege eine Verknüpfung zur `zulip_status.exe` in diesen Ordner

Die App startet dann bei jedem Login, du wählst deinen Status mit einem Klick, und das Fenster schließt sich nach 2 Sekunden von selbst.

---

## Status-Optionen anpassen

Die verfügbaren Status-Optionen sind in `src/main.rs` als Konstante definiert:

```rust
const STATUS_OPTIONS: &[StatusOption] = &[
    StatusOption {
        label: "Im Büro",
        status_text: "Im Büro",
        emoji_name: "office",
        emoji_code: "1f3e2",
    },
    StatusOption {
        label: "Arbeitet von zu Hause",
        status_text: "Arbeitet von zu Hause",
        emoji_name: "house",
        emoji_code: "1f3e0",
    },
];
```

Um einen Eintrag zu ändern oder hinzuzufügen, passe `label`, `status_text`, `emoji_name` und `emoji_code` an. Den Emoji-Code (Unicode) findest du z. B. auf [emojipedia.org](https://emojipedia.org) — verwende den Codepoint ohne `U+`, also z. B. `1f600` für 😀.

---

## Abhängigkeiten

| Crate | Zweck |
|---|---|
| `eframe` / `egui` | GUI-Framework (immediate mode) |
| `reqwest` | HTTP-Client für die Zulip REST API |
| `serde` | JSON-Deserialisierung der API-Antwort |
| `dirs` | Plattformübergreifender Pfad zum Home-Verzeichnis |
