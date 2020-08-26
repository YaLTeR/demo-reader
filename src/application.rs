use std::env;
use std::rc::Rc;

use gio::prelude::*;
use glib::g_debug;
use gtk::prelude::*;
use libhandy as hdy;

use crate::config;
use crate::window::Window;

pub struct Application {
    app: gtk::Application,
    window: Rc<Window>,
}

impl Application {
    pub fn new() -> Self {
        let app =
            gtk::Application::new(Some(config::APP_ID), gio::ApplicationFlags::NON_UNIQUE).unwrap();
        let window = Rc::new(Window::new());

        let application = Self { app, window };

        application.setup_gactions();
        application.setup_signals();
        application
    }

    fn setup_gactions(&self) {
        let action = gio::SimpleAction::new("quit", None);
        action.connect_activate({
            let app = self.app.downgrade();
            move |_, _| {
                let app = app.upgrade().unwrap();
                app.quit();
            }
        });
        self.app.add_action(&action);
        self.app
            .set_accels_for_action("app.quit", &["<primary>q", "Escape"]);
    }

    fn setup_signals(&self) {
        self.app.connect_startup(|_| {
            hdy::init();
        });

        self.app.connect_activate({
            let window = Rc::downgrade(&self.window);
            move |app| {
                let window = window.upgrade().unwrap();
                window.window.set_application(Some(app));
                app.add_window(&window.window);
                window.window.show_all();
            }
        });
    }

    pub fn run(&self) {
        g_debug!(
            config::LOG_DOMAIN,
            "Demo Reader{} ({})",
            config::NAME_SUFFIX,
            config::APP_ID
        );
        g_debug!(
            config::LOG_DOMAIN,
            "Version: {} ({})",
            config::VERSION,
            config::PROFILE
        );
        g_debug!(config::LOG_DOMAIN, "Datadir: {}", config::PKGDATADIR);

        let args: Vec<String> = env::args().collect();
        self.app.run(&args);
    }
}
