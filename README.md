
# rust_qml_template

A minimal **Rust + QML (QtQuick)** template using [cxx-qt](https://github.com/KDAB/cxx-qt) to build native Qt applications with Rust business logic.

---

## 📦 Project Structure

```
.
├── Cargo.toml                # Rust dependencies and metadata
├── build.rs                  # Build script for registering QML modules and linking Qt
├── src/
│   ├── main.rs               # Application entry point
│   └── cxxqt_object.rs       # Rust-QML QObject bridge using cxx-qt
├── qml/
│   └── main.qml              # UI written in QtQuick QML
└── qml.qrc                   # (Optional) Qt resource file for bundling QML
```

---

## 🚀 Getting Started

### 1. Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Qt 6.x](https://www.qt.io/download)
- Set your `QT_DIR` environment variable to your Qt installation path.

**Example:**

```bash
# Linux/macOS
export QT_DIR=/path/to/Qt/6.6.3/gcc_64

# Windows (PowerShell)
$env:QT_DIR = "C:\Qt\6.6.3\msvc2019_64"
```

---

### 2. Build & Run

```bash
cargo run
```

---

## 🧩 How It Works

- `cxx-qt` enables bidirectional communication between Rust and QML.
- Rust structs annotated with `#[qobject]` are exposed to QML as QObject types.
- Properties (`#[qproperty]`) and methods (`#[qinvokable]`) defined in Rust are available in QML.
- The `build.rs` file registers the QML module and links the required Qt modules.

---

## 🛠 Customization

### Change QML Import URI

Edit the URI in both `build.rs` and `qml/main.qml`:

```rust
// build.rs
uri: "com.example.app",
```

```qml
// main.qml
import com.example.app 1.0
```

---

### Add More QML or Rust Files

Update the `QmlModule` section in `build.rs`:

```rust
.qml_module(QmlModule {
    uri: "com.example.app",
    rust_files: &["src/cxxqt_object.rs", "src/another_object.rs"],
    qml_files: &["qml/main.qml", "qml/extra.qml"],
    ..Default::default()
})
```

---

### Define Additional QObjects

Follow the pattern in `cxxqt_object.rs`, and don’t forget to update `build.rs` accordingly.

---

## 📝 License

Licensed under either of:

- MIT License
- Apache License, Version 2.0

---

## 🤝 Credits

- [KDAB](https://www.kdab.com) for developing [cxx-qt](https://github.com/KDAB/cxx-qt)
