use std::os::linux::raw::stat;
use std::process::exit;
use gtk::prelude::*; // Import all necessary traits from gtk3::prelude
use gtk::{Application, Builder, Window, WindowType, Box as GtkBox, gio, SeparatorMenuItem, Menu, CssProvider, StyleContext, gdk};

fn main() {
    let app = Application::new(Some("com.octorrent.rust"), Default::default());

    app.connect_activate(|app| {
        let builder = Builder::from_file("res/ui/gtk3/window.ui");

        let provider = CssProvider::new();
        provider.load_from_path("res/ui/gtk3/style.css").expect("Failed to load CSS file.");

        StyleContext::add_provider_for_screen(
            &gdk::Screen::default().expect("Failed to get default screen."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // Get the window and other widgets from the builder
        let window: Window = builder
            .object("MainWindow")  // Use the ID you set in Glade for the main window
            .expect("Failed to get the 'MainWindow' from window.ui");

        window.set_application(Some(app));

        //let window = Window::new(WindowType::Toplevel);
        window.set_title("OcTorrent");
        window.connect_destroy(|_| exit(0));


        let statusbar: GtkBox = builder
            .object("statusbar")
            .expect("Couldn't find 'statusbar' in window.ui");
        statusbar.set_widget_name("statusbar");




        let builder = Builder::from_file("res/ui/octorrent-ui.xml");
        let menubar: gio::Menu = builder
            .object("main_window_menu")
            .expect("Couldn't find 'main_window_menu' in octorrent-ui.xml");

        app.set_menubar(Some(&menubar));

        window.show_all();
    });

    app.run();
}
