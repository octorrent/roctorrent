
use std::process::exit;
use gtk::prelude::*; // Import all necessary traits from gtk::prelude
use gtk::{CssProvider, Image, Orientation, Window, WindowType};

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

    let window = Window::new(WindowType::Toplevel);
    window.set_title("OcTorrent");
    window.set_default_size(1280, 720);
    window.set_resizable(true);
    window.connect_destroy(|_| exit(0));

    window.show_all();
    gtk::main();
}
