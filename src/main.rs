use gtk::prelude::*;
use gio::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label};

fn main() {
    // demo application
    let application = Application::new(
        Some("com.github.gtk-rs.examples.basic"),
        Default::default(),
    )
    .expect("failed to initialize GTK application");

    // initialize the application
    application.connect_activate(|app| {
        // create a window
        let window = ApplicationWindow::new(app);
        // set the window title
        window.set_title("First GTK+ Program");
        // set the window size
        window.set_default_size(350, 70);

        // create a label
        let label = Label::new(Some("Hello World!"));
        // add the label to the window
        window.add(&label);

        // show the window
        window.show_all();
    });

    // run the application
    application.run(&[]);
}