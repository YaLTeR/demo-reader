use gettextrs::*;

mod application;
mod config;
mod static_resources;
mod window;

use application::Application;
use config::{GETTEXT_PACKAGE, LOCALEDIR};

fn main() {
    // Prepare i18n
    setlocale(LocaleCategory::LcAll, "");
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR);
    textdomain(GETTEXT_PACKAGE);

    glib::set_application_name(&format!("Demo Reader{}", config::NAME_SUFFIX));
    glib::set_prgname(Some("demo-reader"));

    gtk::init().expect("Unable to start GTK3");

    static_resources::init().expect("Failed to initialize the resource file.");

    let app = Application::new();
    app.run();
}
