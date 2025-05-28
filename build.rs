use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        // Link Qt modules
        // - Qt Core is always linked
        // - Qt Gui is linked via `qt_gui` Cargo feature in cxx-qt-lib
        // - Qt Qml is linked via `qt_qml` feature
        // - On macOS, Qt Qml may require linking Qt Network
        .qt_module("Network") // Optional: Remove if not targeting macOS
        .qml_module(QmlModule {
            // Replace with your custom URI
            uri: "com.example.app",
            rust_files: &["src/cxxqt_object.rs"],
            qml_files: &["qml/main.qml"],
            ..Default::default()
        })
        .build();
}
