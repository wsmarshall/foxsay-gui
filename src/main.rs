use gtk::prelude::*;

fn build_ui(app: &gtk::Application) {
    let glade_src = include_str!("layout.glade");
    let builder = gtk::Builder::from_string(&glade_src);

    //get the application, associate it with this 
    // newly defined window
    let window: gtk::Window = builder.object("application_window").unwrap();
    //associate this application with this window
    window.set_application(Some(app));

    //set up inputs
    let message_input: gtk::Entry = builder.object("message_input").unwrap();

    //button for submissions
    let button: gtk::Button = builder.object("generate_btn").unwrap();

    //for displaying outputs
    let message_output: gtk::Label = builder.object("message output").unwrap();
    let image_output: gtk::Image = builder.object("image output").unwrap();

    button.connect_clicked(|_| {
        message_output.set_text(&format!("{}\n \\\n \\", message_input.text().as_str()));
        image_output.show();
    });

    window.show_all();
    image_output.hide();
}

fn main() {
    let application = gtk::Application::new(Some("io.github.wsmarshall.foxsay-gui-glade"), Default::default());

    application.connect_activate(build_ui);
    application.run();

}
