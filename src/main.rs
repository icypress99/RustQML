
// Declare your Rust <-> QML bridge module
pub mod cxxqt_object;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};

fn main() {
    // Create the application and QML engine
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    // Load the QML file (adjust the path based on your project structure)
    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/com/example/app/qml/main.qml"));
        // Tip: You can change the above path to point to your actual QML file.
    }

    // Connect to the QML engine's quit signal
    if let Some(engine) = engine.as_mut() {
        engine
            .as_qqmlengine()
            .on_quit(|_| {
                println!("QML Quit!");
            })
            .release();
    }

    // Start the application event loop
    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
