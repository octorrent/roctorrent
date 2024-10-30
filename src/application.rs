use gtk::ApplicationWindow;
use gtk::gio::SimpleAction;
use gtk::prelude::{ActionMapExt, GtkWindowExt};
use crate::config::VERSION;

pub fn init_actions(window: &ApplicationWindow) {
    let action = SimpleAction::new("show-about-dialog", None);
    let window_clone = window.clone();
    action.connect_activate(move |_, _| {
        show_about(&window_clone);
    });
    window.add_action(&action);
}

pub fn show_about(window: &ApplicationWindow) {
    let dialog = gtk::AboutDialog::builder()
        .transient_for(window)
        .modal(true)
        .program_name("OcTorrent")
        .version(VERSION)
        .authors(vec!["DrBrad"])
        //.authors(vec![<&str as Into<T>>::into("DrBrad")])
        .build();

    dialog.present();
}