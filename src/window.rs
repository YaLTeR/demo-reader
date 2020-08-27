use std::fs::File;
use std::rc::Rc;
use std::str;

use gio::prelude::*;
use gtk::prelude::*;
use hdy::HeaderBarExt;
use libhandy as hdy;
use memmap::Mmap;

use crate::config::PROFILE;

pub struct Window {
    pub window: hdy::ApplicationWindow,
    header_bar: hdy::HeaderBar,
    stack: gtk::Stack,
    label_game: gtk::Label,
    label_map: gtk::Label,
    label_time: gtk::Label,
}

impl Window {
    pub fn new() -> Self {
        let builder = gtk::Builder::from_resource("/io/github/yalter/DemoReader/window.ui");
        let window: hdy::ApplicationWindow = builder.get_object("window").unwrap();
        let header_bar: hdy::HeaderBar = builder.get_object("header_bar").unwrap();
        let stack: gtk::Stack = builder.get_object("stack").unwrap();
        let label_game: gtk::Label = builder.get_object("label_game").unwrap();
        let label_map: gtk::Label = builder.get_object("label_map").unwrap();
        let label_time: gtk::Label = builder.get_object("label_time").unwrap();

        // Devel Profile
        if PROFILE == "Devel" {
            // TODO: uncomment when there's a non-Devel release.
            // window.get_style_context().add_class("devel");
        }

        Window {
            window,
            header_bar,
            stack,
            label_game,
            label_map,
            label_time,
        }
    }

    pub fn open(self: &Rc<Self>, file: Option<gio::File>) {
        if file.is_none() {
            return;
        }

        let file = file.unwrap();

        self.stack.set_visible_child_name("page_main");

        let context = glib::MainContext::default();

        let future = {
            let self_ = self.clone();
            let file = file.clone();
            async move {
                let info = file
                    .query_info_async_future(
                        "standard::display-name",
                        gio::FileQueryInfoFlags::NONE,
                        glib::PRIORITY_DEFAULT,
                    )
                    .await;

                let name = if let Ok(info) = info {
                    info.get_display_name()
                } else {
                    file.get_parse_name()
                };

                self_.header_bar.set_subtitle(name.as_deref());
            }
        };

        context.spawn_local(future);

        if let Some(path) = file.get_path() {
            // We have a path so we can mmap the file.

            // TODO: errors.
            let file = File::open(path).unwrap();
            let contents = unsafe { Mmap::map(&file).unwrap() };

            self.on_demo_loaded(&contents);
        } else {
            // No path (e.g. file over the network). Load contents manually.

            // TODO: async, errors.
            let (contents, _) = file.load_contents(None::<&gio::Cancellable>).unwrap();

            self.on_demo_loaded(&contents);
        }
    }

    fn on_demo_loaded(&self, contents: &[u8]) {
        // TODO: errors.
        let demo = hldemo::Demo::parse_without_frames(contents).unwrap();

        self.on_demo_header_parsed(&demo);
    }

    fn on_demo_header_parsed(&self, demo: &hldemo::Demo) {
        self.label_game.set_label(to_str(&demo.header.game_dir));
        self.label_map.set_label(to_str(&demo.header.map_name));

        let seconds = demo
            .directory
            .entries
            .iter()
            .filter(|e| e.entry_type != 0)
            .fold(0f32, |acc, e| acc + e.track_time);
        self.label_time.set_label(&format_time(seconds));
    }
}

// TODO: errors.
fn to_str(x: &[u8]) -> &str {
    let zero = x.iter().position(|x| *x == 0).unwrap_or(x.len());
    str::from_utf8(&x[..zero]).unwrap()
}

fn format_time(seconds: f32) -> String {
    let hours = (seconds / 3600.) as u32;
    let seconds = seconds % 3600.;
    let minutes = (seconds / 60.) as u8;
    let seconds = seconds % 60.;
    let fractional = ((seconds % 1.) * 1000.) as u16;
    let seconds = seconds as u8;

    if hours == 0 && minutes == 0 {
        format!("{}.{:03}", seconds, fractional)
    } else if hours == 0 {
        format!("{}:{:02}.{:03}", minutes, seconds, fractional)
    } else {
        format!("{}:{:02}:{:02}.{:03}", hours, minutes, seconds, fractional)
    }
}
