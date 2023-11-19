use gdk::keys::Key;
use gtk::prelude::*;
use gio::prelude::*;
use gtk::{Application, ApplicationWindow, DrawingArea};

struct Dimensions {
    width: i32,
    height: i32,
}
struct Coordinates {
    x: f32,
    y: f32,
}

const GAME_DIMENSIONS: Dimensions = Dimensions {
    width: 1280,
    height: 720,
};

const PLAYER_DIMENSIONS: Dimensions = Dimensions {
    width: 100,
    height: 100,
};

fn main() {
    // demo application
    let application = Application::new(
        Some("com.github.tohrxyz.flappy_rustie"),
        Default::default(),
    )
    .expect("failed to initialize GTK application");

    // initialize the application
    application.connect_activate(|app| {
        // create a window
        let window = ApplicationWindow::new(app);
        // set the window id
        window.set_widget_name("window");
        // set the window title
        window.set_title("Flappy Rustie");
        // set the window size
        window.set_default_size(GAME_DIMENSIONS.width, GAME_DIMENSIONS.height);

        let drawing_area = DrawingArea::new();
        drawing_area.set_widget_name("rectangle");
        drawing_area.set_size_request(200, 50);

        const BOTTOM: Coordinates = Coordinates { x: 0.0, y: GAME_DIMENSIONS.height as f32 - PLAYER_DIMENSIONS.height as f32 };

        // Handle drawing the rectangle on the DrawingArea
        drawing_area.connect_draw(|_, cr| {
            cr.set_source_rgb(0.0, 0.0, 0.0); // Set color (black in this case)
            cr.rectangle(
                0.0, 
                BOTTOM.y as f64,
                PLAYER_DIMENSIONS.width as f64, 
                PLAYER_DIMENSIONS.height as f64
            ); // Create a rectangle
            cr.fill(); // Fill the rectangle

            Inhibit(false)
        });

        window.add(&drawing_area);

        // set the window background color
        let screen = window.get_screen().expect("Couldn't get screen");
        let provider = gtk::CssProvider::new();
        provider
            .load_from_data(
                b"
                #window {
                    background-color: #89B8CA;
                }
                ",
            )
            .expect("Failed to load CSS");
        gtk::StyleContext::add_provider_for_screen(
            &screen,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        gtk::main_iteration();

        window.connect_key_press_event(|window, event| {
            let pressed_key = event.get_keycode().unwrap() as u32;
            let key = Key::from(pressed_key);
            let space = Key::from(49);

            if key == space {
                println!("Space pressed");
            }
            
            println!("Key pressed: {:?}", key);
            gtk::Inhibit(false)
        });

        // show the window
        window.show_all();
    });

    // run the application
    application.run(&[]);
}
