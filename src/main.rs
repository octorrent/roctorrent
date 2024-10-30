mod config;
mod application;

use std::process::exit;
use gtk::prelude::*;
use gtk::{Application, Builder, Box as GtkBox, gio, CssProvider, StyleContext, gdk, ApplicationWindow};
use gtk::gdk_pixbuf::PixbufLoader;
use crate::application::init_actions;

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

        let window: ApplicationWindow = builder
            .object("MainWindow")
            .expect("Failed to get the 'MainWindow' from window.ui");

        window.set_application(Some(app));


        let svg_data = include_bytes!("../res/favicon.svg");
        let loader = PixbufLoader::with_type("svg").expect("Failed to create SVG loader");
        loader.write(svg_data).expect("Failed to load SVG data");
        loader.close().expect("Failed to close SVG loader");
        let icon_pixbuf = loader.pixbuf().expect("Failed to get Pixbuf from SVG");

        window.set_icon(Some(&icon_pixbuf));



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

        init_actions(&window);

        window.show_all();
    });

    app.run();
}
