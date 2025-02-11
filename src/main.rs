mod config;
mod application;
mod handlers;

use std::process::exit;
use gtk::prelude::*;
use gtk::{Application, Builder, gio, CssProvider, StyleContext, gdk, ApplicationWindow, ListBox, ListBoxRow, Label, Orientation, ScrolledWindow, Image, ProgressBar};
use crate::application::init_actions;
use crate::handlers::torrent::Torrent;

fn main() {
    let app = Application::new(Some("com.sectorrent.rust"), Default::default());

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

        /*
        let svg_data = include_bytes!("../res/ic_launcher.svg");
        let loader = PixbufLoader::with_type("svg").expect("Failed to create SVG loader");
        loader.write(svg_data).expect("Failed to load SVG data");
        loader.close().expect("Failed to close SVG loader");
        let icon_pixbuf = loader.pixbuf().expect("Failed to get Pixbuf from SVG");

        window.set_icon(Some(&icon_pixbuf));
        */
        window.set_icon_from_file("res/images/ic_launcher.svg").expect("Failed to load icon");

        //let window = Window::new(WindowType::Toplevel);
        window.set_title("SecTorrent - Rust");
        window.connect_destroy(|_| exit(0));





        //ALL

        let list_box = ListBox::new();
        list_box.add(&create_row(Torrent::new("ubuntu-21.10-desktop-amd64.iso")));

        let all_tab_layout: ScrolledWindow = builder
            .object("all_tab_layout")
            .expect("Couldn't find 'all_tab_layout' in window.ui");

        all_tab_layout.add(&list_box);
        list_box.show_all();


        //DOWNLOADING

        let list_box = ListBox::new();
        list_box.add(&create_row(Torrent::new("ubuntu-21.10-desktop-amd64.iso")));

        let downloading_tab_layout: ScrolledWindow = builder
            .object("downloading_tab_layout")
            .expect("Couldn't find 'downloading_tab_layout' in window.ui");

        downloading_tab_layout.add(&list_box);
        list_box.show_all();


        //COMPLETE

        let complete_tab_layout: ScrolledWindow = builder
            .object("complete_tab_layout")
            .expect("Couldn't find 'complete_tab_layout' in window.ui");

        complete_tab_layout.add(&create_no_torrents());



        let builder = Builder::from_file("res/ui/sectorrent-ui.xml");
        let menubar: gio::Menu = builder
            .object("main_window_menu")
            .expect("Couldn't find 'main_window_menu' in sectorrent-ui.xml");

        app.set_menubar(Some(&menubar));

        init_actions(&app, &window);

        window.show_all();
    });

    app.run();
}

fn create_no_torrents() -> gtk::Box {
    let vbox = gtk::Box::new(Orientation::Vertical, 5);
    vbox.set_widget_name("no_torrents_layout");

    let image = Image::new();
    image.set_from_file(Some("res/images/background_error_no_torrents.svg"));
    vbox.pack_start(&image, false, false, 0);

    let label = Label::new(Some("Uh oh,"));
    label.set_widget_name("title");
    vbox.pack_start(&label, false, true, 0);

    let label = Label::new(Some("You don't have any torrents"));
    label.set_widget_name("description");
    vbox.pack_start(&label, false, true, 0);

    vbox
}

fn create_row(torrent: Torrent) -> ListBoxRow {
    let row = ListBoxRow::new();
    let hbox = gtk::Box::new(Orientation::Horizontal, 5);

    let icon = Image::new();
    icon.set_from_file(Some("res/images/ic_file_iso.svg"));
    hbox.pack_start(&icon, false, false, 0);

    let vbox = gtk::Box::new(Orientation::Vertical, 5);

    let label = Label::new(Some(torrent.get_title().as_str()));
    label.set_widget_name("title");
    label.set_halign(gtk::Align::Start);
    vbox.pack_start(&label, false, true, 0);

    let label = Label::new(Some("624 MB of 3.12 GB - 2 min, 13 secs left"));
    label.set_widget_name("status");
    label.set_halign(gtk::Align::Start);
    vbox.pack_start(&label, false, true, 0);

    let progress = ProgressBar::new();
    progress.set_fraction(0.5);
    progress.set_expand(true);
    vbox.pack_start(&progress, true, true, 0);

    let label = Label::new(Some("Downloading from 45 of 50 connected peers"));
    label.set_widget_name("description");
    label.set_halign(gtk::Align::Start);
    vbox.pack_start(&label, false, true, 0);

    hbox.pack_start(&vbox, true, true, 5);

    row.add(&hbox);
    row
}
