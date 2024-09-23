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
    let message_output: gtk::Label = builder.object("message_output").unwrap();
    let message_output_clone = message_output.clone(); //only a shallow clone of the pointer
    
    let death_switch: gtk::Switch = builder.object("is_dead_switch").unwrap();
    
    let image_output: gtk::Image = builder.object("image_output").unwrap();
    let image_output_clone = image_output.clone(); //sim. shallow clone


    button.connect_clicked(move |_| {
        message_output_clone.set_text(&format!("{}\n \\\n \\", message_input_clone.text().as_str()));
        
        let is_dead = death_switch.is_active();
        if is_dead {
            image_output_clone.set_from_file(
                Some("./images/dead_fox.jpg")
            )
        } else {
            image_output_clone.set_from_file(Some("./images/fox.png"))
        }
        
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
