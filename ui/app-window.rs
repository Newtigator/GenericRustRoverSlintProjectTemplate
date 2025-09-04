slint::slint! {
import { Button, VerticalBox } from "std-widgets.slint";
export component MainWindow inherits Window {

    title: "Dogbo";
    width: 300px;
    height: 300px;

    Text {
        x: 100px;
        y: 40px;
        text: "hello world";
        color: blue;
    }
    Button{
       x: 10px;
       y: 40px;
       text: "Fuck You";
    }
     Rectangle {
            x: 10px; y: 10px;
            width: 10px;
            height: 10px;
            background: #315afd;
        }

            Rectangle {
                x: 10px; y: 60px;
                width: 10px;
                height: 10px;
                background: green;
                border-width: 2px;
                border-color: red;
            }

            // Transparent Rectangle with a border and a radius
            Rectangle {
                x: 10px; y: 90px;
                width: 30px;
                height: 30px;
                border-width: 1px;
                border-color: black;
                border-radius: 5px;
            }

            // A radius of width/2 makes it a circle
            Rectangle {
                x: 10px; y: 120px;
                width: 10px;
                height: 10px;
                background: yellow;
                border-width: 2px;
                border-color: blue;
                border-radius: self.width/2;
            }

                MenuBar {
                    Menu {
                        title: @tr("File");
                        MenuItem {
                            title: @tr("New");
                            activated => { file-new(); }
                        }
                        MenuItem {
                            title: @tr("Open");
                            activated => { file-open(); }
                        }
                    }
                    Menu {
                        title: @tr("Edit");
                        MenuItem {
                            title: @tr("Copy");
                        }
                        MenuItem {
                            title: @tr("Paste");
                        }
                        MenuSeparator {}
                        Menu {
                            title: @tr("Find");
                            MenuItem {
                                title: @tr("Find in document...");
                            }
                            MenuItem {
                                title: @tr("Find Next");
                            }
                            MenuItem {
                                title: @tr("Find Previous");
                            }
                        }
                    }
                }

                callback file-new();
                callback file-open();




}//end entry

}
