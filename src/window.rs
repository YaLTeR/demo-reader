use gtk::prelude::*;

use crate::config::PROFILE;

pub struct Window {
    pub window: gtk::ApplicationWindow,
}

impl Window {
    pub fn new() -> Self {
        let builder = gtk::Builder::from_resource("/io/github/yalter/DemoReader/window.ui");
        let window: gtk::ApplicationWindow = builder.get_object("window").unwrap();

        // Devel Profile
        if PROFILE == "Devel" {
            window.get_style_context().add_class("devel");
        }

        Window { window }
    }
}
