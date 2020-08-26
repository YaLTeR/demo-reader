use gio::prelude::SettingsExt;
use gtk::prelude::GtkWindowExt;

pub fn load(window: &gtk::ApplicationWindow, settings: &gio::Settings) {
    let width = settings.get_int("window-width");
    let height = settings.get_int("window-height");

    if width > -1 && height > -1 {
        window.resize(width, height);
    }

    let x = settings.get_int("window-x");
    let y = settings.get_int("window-y");
    let is_maximized = settings.get_boolean("is-maximized");

    if x > -1 && y > -1 {
        window.move_(x, y);
    } else if is_maximized {
        window.maximize();
    }
}

pub fn save(window: &gtk::ApplicationWindow, settings: &gio::Settings) -> Result<(), glib::BoolError> {
    let size = window.get_size();
    let position = window.get_position();

    settings.set_int("window-width", size.0)?;
    settings.set_int("window-height", size.1)?;

    settings.set_boolean("is-maximized", window.is_maximized())?;

    settings.set_int("window-x", position.0)?;
    settings.set_int("window-y", position.1)?;

    Ok(())
}
