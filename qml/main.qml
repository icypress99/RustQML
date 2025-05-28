import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Window 2.12

// Replace this with your actual Rust-registered QML module
import com.example.app 1.0

ApplicationWindow {
    id: root
    width: 640
    height: 480
    visible: true
    title: qsTr("Rust + QML Template")
    color: palette.window

    // Rust-exposed object (from cxx-qt bridge)
    AppObject {
        id: appObject
        number: 1
        string: qsTr("My String with number: %1").arg(number)
    }

    Column {
        anchors.fill: parent
        anchors.margins: 10
        spacing: 10

        Label {
            text: qsTr("Number: %1").arg(appObject.number)
            color: palette.text
        }

        Label {
            text: qsTr("String: %1").arg(appObject.string)
            color: palette.text
        }

        Button {
            text: qsTr("Increment Number")
            onClicked: appObject.incrementNumber()
        }

        Button {
            text: qsTr("Say Hi!")
            onClicked: appObject.sayHi(appObject.string, appObject.number)
        }

        Button {
            text: qsTr("Quit")
            onClicked: Qt.quit()
        }
    }
}
