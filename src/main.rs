use std::process::exit;
use gtk::prelude::*; // Import all necessary traits from gtk3::prelude
use gtk::{Application, Builder, Window, WindowType, Box as GtkBox, gio, SeparatorMenuItem};
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

        /*
        let window_layout: GtkBox = builder
            .object("window_layout")
            .expect("Couldn't find the layout in the .ui file");
        //window_layout.pack_start(&menu_bar, false, false, 0);
        */

        let menubar = {
            let file_menu = {
                let open_menu_item = gio::MenuItem::new(Some("Open"), Some("app.open")); //CTRL+O
                let open_magnet_menu_item = gio::MenuItem::new(Some("Open Magnet"), Some("app.open_magnet")); //CTRL+U
                let new_menu_item = gio::MenuItem::new(Some("New..."), Some("app.new")); //CTRL+N
                let start_all_menu_item = gio::MenuItem::new(Some("Start All"), Some("app.start_all"));
                let pause_all_menu_item = gio::MenuItem::new(Some("Pause All"), Some("app.pause_all"));
                let quit_menu_item = gio::MenuItem::new(Some("Quit"), Some("app.quit")); //CTRL+Q

                let file_menu = gio::Menu::new();
                file_menu.append_item(&open_menu_item);
                file_menu.append_item(&open_magnet_menu_item);
                file_menu.append_item(&new_menu_item);
                //file_menu.append_item(SeparatorMenuItem::new());
                file_menu.append_item(&start_all_menu_item);
                file_menu.append_item(&pause_all_menu_item);
                //file_menu.append_item(SeparatorMenuItem::new());
                file_menu.append_item(&quit_menu_item);
                file_menu
            };

            let edit_menu = {
                let select_all_menu_item = gio::MenuItem::new(Some("Select All"), Some("app.select_all")); //CTRL+A
                let deselect_all_menu_item = gio::MenuItem::new(Some("Deselect All"), Some("app.deselect_all")); //CTRL+SHIFT+A
                let preferences_menu_item = gio::MenuItem::new(Some("Preferences"), Some("app.preferences")); //CTRL+SHIFT+A

                let edit_menu = gio::Menu::new();
                edit_menu.append_item(&select_all_menu_item);
                edit_menu.append_item(&deselect_all_menu_item);
                //file_menu.append_item(SeparatorMenuItem::new());
                edit_menu.append_item(&preferences_menu_item);
                edit_menu
            };

            let torrent_menu = {
                let torrent_menu = gio::Menu::new();
                torrent_menu
            };

            let view_menu = {
                let view_menu = gio::Menu::new();
                view_menu
            };

            let help_menu = {
                let about_menu_item = gio::MenuItem::new(Some("About"), Some("app.about"));

                let help_menu = gio::Menu::new();
                help_menu.append_item(&about_menu_item);
                help_menu
            };

            let menubar = gio::Menu::new();
            menubar.append_submenu(Some("File"), &file_menu);
            menubar.append_submenu(Some("Edit"), &edit_menu);
            menubar.append_submenu(Some("Torrent"), &torrent_menu);
            menubar.append_submenu(Some("View"), &view_menu);
            menubar.append_submenu(Some("Help"), &help_menu);

            menubar
        };

        app.set_menubar(Some(&menubar));

        window.show_all();
    });

    app.run();
}
