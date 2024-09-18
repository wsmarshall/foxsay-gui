use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Image, Label, Orientation};

fn main() {
    let app = Application::new(Some("io.github.wsmarshall.foxsay-gui"), Default::default());

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Foxsay");
        window.set_default_size(1300, 1000);

        let layout_box = GtkBox::new(Orientation::Vertical, 0);

        let label = Label::new(Some("Wa-pa-pa-pa-pa-pa-pow!\n   \\\n    \\"));
        layout_box.add(&label);

        let fox_image = Image::from_file("./images/fox.jpeg");
        layout_box.add(&fox_image);

        window.add(&layout_box);

        window.show_all();
    });

    app.run();
}
