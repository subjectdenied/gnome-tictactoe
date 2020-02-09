use gtk::prelude::*;
use gio::prelude::*;
use gettextrs::*;
use std::env;

mod config;
mod window;
use crate::window::Window;

fn main() {
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));

    setlocale(LocaleCategory::LcAll, "");
    bindtextdomain("gnome-tictactoe", config::LOCALEDIR);
    textdomain("gnome-tictactoe");

    let res = gio::Resource::load(config::PKGDATADIR.to_owned() + "/gnome-tictactoe.gresource")
                                .expect("Could not load resources");
    gio::resources_register(&res);

    // css
    const STYLE: &str = "
        #game_grid {
            background-color: grey;
            font-size: 24px;
        }

        #game_grid button {
            border-left: 1px solid black;
            border-top: 1px solid black;
            border-right: 1px solid black;
            border-bottom: 1px solid black;
            background-color: white;
            font-size: 40px;
            font-family: Courier New;
        }

        button .turn1 {
            color: green;
        }

        button .turn2 {
            color: red;
        }

        label .turn1 {
            color: green;
        }

        label .turn2 {
            color: red;
        }
    ";

    let app = gtk::Application::new(Some("com.subjectdenied.gnome-tictactoe"), Default::default()).unwrap();
    app.connect_activate(move |app| {
        let window = Window::new();

        let provider = gtk::CssProvider::new();
        provider
            .load_from_data(STYLE.as_bytes())
            .expect("Failed to load CSS");
        // We give the CssProvided to the default screen so the CSS rules we added
        // can be applied to our window.
        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        window.widget.set_application(Some(app));
        app.add_window(&window.widget);
        window.widget.present();
    });


    let args: Vec<String> = env::args().collect();
    app.run(&args);

}

