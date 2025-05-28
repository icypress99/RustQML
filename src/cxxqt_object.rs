/// The bridge definition for our QObject (Rust <-> QML)
#[cxx_qt::bridge]
pub mod qobject {

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        /// Alias to the QString type used in QML
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        // Define the QObject exposed to QML
        #[qobject]
        #[qml_element]
        #[qproperty(i32, number)]
        #[qproperty(QString, string)]
        #[namespace = "app_object"]
        type AppObject = super::AppObjectRust;
    }

    extern "RustQt" {
        // Invokable methods accessible from QML
        #[qinvokable]
        #[cxx_name = "incrementNumber"]
        fn increment_number(self: Pin<&mut AppObject>);

        #[qinvokable]
        #[cxx_name = "sayHi"]
        fn say_hi(self: &AppObject, string: &QString, number: i32);
    }
}

use core::pin::Pin;
use cxx_qt_lib::QString;

/// The backing Rust struct for our QObject
#[derive(Default)]
pub struct AppObjectRust {
    number: i32,
    string: QString,
}

impl qobject::AppObject {
    /// Increment the `number` Q_PROPERTY
    pub fn increment_number(self: Pin<&mut Self>) {
        let previous = *self.number();
        self.set_number(previous + 1);
    }

    /// Example method callable from QML
    pub fn say_hi(&self, string: &QString, number: i32) {
        println!("Hi from Rust! String is '{string}' and number is {number}");
    }
}
