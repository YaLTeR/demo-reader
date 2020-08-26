use gtk::prelude::*;
use libhandy as hdy;

use crate::config::PROFILE;

pub struct Window {
    pub window: hdy::ApplicationWindow,
}

impl Window {
    pub fn new() -> Self {
        let builder = gtk::Builder::from_resource("/io/github/yalter/DemoReader/window.ui");
        let window: hdy::ApplicationWindow = builder.get_object("window").unwrap();

        // Devel Profile
        if PROFILE == "Devel" {
            // TODO: uncomment when there's a non-Devel release.
            // window.get_style_context().add_class("devel");
        }

        Window { window }
    }

    pub fn open(&self, _file: Option<gio::File>) {
    }
}
