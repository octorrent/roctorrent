use std::process::exit;
use gtk::prelude::*; // Import all necessary traits from gtk3::prelude
use gtk::{Builder, CssProvider, Image, Orientation, Window, WindowType};

/*
sudo apt update
sudo apt install -y libglib2.0-dev
sudo apt install -y libcairo2-dev
sudo apt install -y libpango1.0-dev
sudo apt install -y libatk1.0-dev
sudo apt install -y libgdk-pixbuf-2.0-dev
sudo apt install -y libgtk-3-dev

*/

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let builder = Builder::from_file("res/ui/gtk3/window.ui");

    // Get the window and other widgets from the builder
    let window: Window = builder
        .object("main_window")  // Use the ID you set in Glade for the main window
        .expect("Failed to get the main_window from layout.ui");


    //let window = Window::new(WindowType::Toplevel);
    window.set_title("OcTorrent");
    window.connect_destroy(|_| exit(0));

    window.show_all();
    gtk::main();
}
