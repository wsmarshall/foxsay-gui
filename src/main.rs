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
    let message_input_clone = message_input.clone();//shallow clone

    //button for submissions
    let button: gtk::Button = builder.object("generate_btn").unwrap();

    //for displaying outputs
    let message_output: gtk::Label = builder.object("message output").unwrap();
    let message_output_clone = message_output.clone(); //only a shallow clone of the pointer
    let image_output: gtk::Image = builder.object("image output").unwrap();
    let image_output_clone = image_output.clone(); //sim. shallow clone


    button.connect_clicked(move |_| {
        message_output_clone.set_text(&format!("{}\n \\\n \\", message_input_clone.text().as_str()));
        image_output_clone.show();
    });

    window.show_all();
    image_output.hide();
}

fn main() {
    let application = gtk::Application::new(Some("io.github.wsmarshall.foxsay-gui-glade"), Default::default());

    application.connect_activate(build_ui);
    application.run();

}
