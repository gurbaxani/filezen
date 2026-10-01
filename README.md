# FileZen ⚡

> Effortlessly organize your cluttered folders into categorized subdirectories in milliseconds.

[![Release](https://img.shields.io/github/v/release/gurbaxani/filezen?color=brightgreen&label=Release)](https://github.com/gurbaxani/filezen/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey.svg)](https://github.com/gurbaxani/filezen/releases)

---

## 🚀 Quick Install (Linux & macOS)

Anyone can install FileZen in seconds without needing Rust, Cargo, or any build tools:

```bash
curl -fsSL https://raw.githubusercontent.com/gurbaxani/filezen/trunk/install.sh | bash
```

The installer detects your platform, sets up the binary, and makes `filezen` immediately accessible from your terminal.

---

## 📦 Pre-Built Standalone Binaries

If you prefer to download the standalone binary directly, grab the build for your system from the [Releases](https://github.com/gurbaxani/filezen/releases) page:

| Operating System | Architecture | Archive |
| :--- | :--- | :--- |
| **Linux** | x86_64 (64-bit Intel/AMD) | [filezen-x86_64-unknown-linux-musl.tar.gz](https://github.com/gurbaxani/filezen/releases/latest) |
| **Linux** | ARM64 / AArch64 (Raspberry Pi, ARM servers) | [filezen-aarch64-unknown-linux-musl.tar.gz](https://github.com/gurbaxani/filezen/releases/latest) |
| **macOS** | Apple Silicon (M1/M2/M3/M4) | [filezen-aarch64-apple-darwin.tar.gz](https://github.com/gurbaxani/filezen/releases/latest) |
| **macOS** | Intel x86_64 | [filezen-x86_64-apple-darwin.tar.gz](https://github.com/gurbaxani/filezen/releases/latest) |
| **Windows** | x86_64 | [filezen-x86_64-pc-windows-msvc.zip](https://github.com/gurbaxani/filezen/releases/latest) |

Simply extract the archive and place `filezen` in your `PATH`.

---

## 🛠️ Build from Source (Cargo)

If you have Rust installed and want to install directly from source:

```bash
# Install directly from GitHub
cargo install --git https://github.com/gurbaxani/filezen

# Or build locally
git clone https://github.com/gurbaxani/filezen.git
cd filezen
make install
```

---

## 📖 How You Use FileZen

FileZen is designed to be completely intuitive. You can clean any folder on your computer in one command:

### 1. Clean your current directory
```bash
filezen
```

### 2. Clean a specific cluttered directory (e.g. Downloads, Desktop)
```bash
filezen ~/Downloads
filezen ~/Desktop
filezen /path/to/any/messy/folder
```

### 3. View command options & help
```bash
filezen --help
```

---

## 🗂️ How Your Files Are Categorized

FileZen automatically groups common file formats into clean, logical subdirectories:

| Category | Example Extensions |
| :--- | :--- |
| **`Images/`** | `.jpg`, `.jpeg`, `.png`, `.webp`, `.gif`, `.bmp`, `.svg`, `.ico`, `.tiff`, `.heic`, `.raw` |
| **`Documents/`** | `.pdf`, `.docx`, `.doc`, `.txt`, `.rtf`, `.odt`, `.xlsx`, `.xls`, `.pptx`, `.ppt`, `.csv`, `.md` |
| **`Videos/`** | `.mp4`, `.mkv`, `.avi`, `.mov`, `.wmv`, `.flv`, `.webm`, `.m4v`, `.mpg` |
| **`Audio/`** | `.mp3`, `.wav`, `.flac`, `.aac`, `.ogg`, `.m4a`, `.wma`, `.opus` |
| **`Archives/`** | `.zip`, `.tar.gz`, `.tar.bz2`, `.tar.xz`, `.tar`, `.gz`, `.7z`, `.rar`, `.iso` |
| **`Code/`** | `.rs`, `.py`, `.js`, `.ts`, `.html`, `.css`, `.json`, `.yaml`, `.c`, `.cpp`, `.go`, `.sh` |
| **`Misc/`** | Any unrecognized extensions or files without extensions (e.g. `LICENSE`) |

---

## 🛡️ Safety Guarantees

* **Zero File Loss (Collision Protection):** If a file with the same name already exists in the destination folder, FileZen renames the incoming file (e.g. `document_1.pdf`) so your existing work is never overwritten.
* **Directory Skipping:** FileZen only moves loose files. It never tampers with nested folders or project directories.
* **Non-Panicking Resilience:** If a file is locked or requires special permissions, FileZen prints a clear warning to your terminal and proceeds with the rest of your files without crashing.
* **Atomic & Cross-Device Compatible:** Moves files instantaneously on the same volume and provides seamless copy-and-remove fallback across separate mount points.

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).
