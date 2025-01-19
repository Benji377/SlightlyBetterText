[![Test project](https://github.com/Benji377/SlightlyBetterText/actions/workflows/tests.yml/badge.svg)](https://github.com/Benji377/SlightlyBetterText/actions/workflows/tests.yml)
![GitHub downloads](https://img.shields.io/github/downloads/Benji377/SlightlyBetterText/total?label=Downloads)

# :pencil: SBT - SlightlyBetterText

<div align="center">
  <img src="https://raw.githubusercontent.com/Benji377/SlightlyBetterText/refs/heads/main/src/assets/images/logo.svg" alt="Logo" style="width: 45%; max-width: 400px; vertical-align: middle;">
</div>

---

## :star: Introduction
**SlightlyBetterText (SBT)** is a simple, lightweight text editor written in Rust. It's designed with a focus on **simplicity** and **performance**.

The goal of SBT is to provide a minimally functional text editor that uses as few system resources as possible while maintaining essential text editing capabilities. It is cross-platform and supports all three major operating systems: **Windows**, **Linux**, and **macOS**.

---

## :rocket: Usage
Using SBT is straightforward:
- Launch the app by executing it.
- Once open, toggle visibility with `CTRL + ALT + SPACE`.
- Customize your experience with a `settings.json` file (see the [Settings](#gear-settings) section below).

### Features
- Minimalist design for quick text editing.
- Syntax highlighting for certain file types (e.g., Markdown).
- Designed to run in the background and awaken on demand using the shortcut.

---

## :gear: Settings
You can customize SBT by editing the `settings.json` file located in the application directory. 

### Available Options:
- **`startup_file_path`**: The absolute path to a file that will be opened at startup.  
  _Default_: A `.txt` file in your Documents folder.
  
- **`theme`**: The editor's theme. Available options are: `"eighties"`, `"mocha"`, `"ocean"`, `"github"`, `"solarized"`.  
  _Default_: `"solarized"`

- **`word_wrap`**: Toggle word wrapping (`true` or `false`).  
  _Default_: `true`

### Settings File Location:
- **Linux**: `$XDG_CONFIG_HOME/slightlybettertext` or `$HOME/.config/slightlybettertext`
- **Windows**: `{FOLDERID_RoamingAppData}/slightlybettertext/config`
- **macOS**: `$HOME/Library/Application Support/slightlybettertext`

### Example `settings.json` File:
```json
{
  "startup_file_path": "C:\\Users\\<user>\\Documents\\sbt_notes.txt",
  "theme": "solarized",
  "word_wrap": true
}
```

---

## :books: Additional Notes
- **Development Environment**: SBT is primarily developed on **Windows**, with Linux and macOS support being theoretical for now.
- For a roadmap and planned features, see the [TODOs](TODO.md).

---

> **Feedback and contributions are welcome!**  
> Feel free to open issues or submit pull requests to help make SBT even better.
