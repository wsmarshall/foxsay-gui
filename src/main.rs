use gtk::prelude::*;

fn build_ui(app: &gtk::Application) {
    let glade_src = include_str!("layout.glade");
    let builder = gtk::Builder::from_string(&glade_src);

    //get the application, associate it with this 
    // newly defined window
    let window: gtk::Window = builder.object("application_window").unwrap();
    //associate this application with this window
    window.set_application(Some(app));

    window.show_all();

}

fn main() {
    let application = gtk::Application::new(Some("io.github.wsmarshall.foxsay-gui-glade"), Default::default());

    application.connect_activate(build_ui);
    application.run();

}
