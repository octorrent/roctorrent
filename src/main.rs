use std::process::exit;
use gtk::prelude::*; // Import all necessary traits from gtk3::prelude
use gtk::{Application, Builder, Window, WindowType, Box as GtkBox, gio, SeparatorMenuItem, Menu};

fn main() {
    let app = Application::new(Some("com.octorrent.rust"), Default::default());

    app.connect_activate(|app| {
        let builder = Builder::from_file("res/ui/gtk3/window.ui");

        // Get the window and other widgets from the builder
        let window: Window = builder
            .object("MainWindow")  // Use the ID you set in Glade for the main window
            .expect("Failed to get the main_window from layout.ui");

        window.set_application(Some(app));

        //let window = Window::new(WindowType::Toplevel);
        window.set_title("OcTorrent");
        window.connect_destroy(|_| exit(0));

        let builder = Builder::from_file("res/ui/octorrent-ui.xml");
        let menubar: gio::Menu = builder
            .object("main_window_menu")
            .expect("Couldn't find 'main_window_menu' in menu.ui");

        app.set_menubar(Some(&menubar));

        window.show_all();
    });

    app.run();
}
