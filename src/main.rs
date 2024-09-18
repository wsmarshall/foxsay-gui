use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

fn main() {
    let app = Application::new(Some("io.github.wsmarshall.foxsay-gui"), Default::default());

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Foxsay");
        window.set_default_size(451, 82);

        window.show_all();
    });

    app.run();
}
