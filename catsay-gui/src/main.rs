use gtk::{Application, ApplicationWindow, BoxLayout, Image, glib};
use gtk::{Box as GtkBox, Label, Orientation, prelude::*};

const APP_ID: &str = "com.gtk_rs.ikuru.catsay-gui";
fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Catsay")
        .default_width(120)
        .default_height(350)
        .build();

    let gtkbox = GtkBox::new(Orientation::Vertical, 10);
    let layout = BoxLayout::new(Orientation::Vertical);

    gtkbox.set_layout_manager(Some(&layout));

    let label = Label::new(Some("Meow!\n \\\n \\"));

    gtkbox.append(&label);

    let cat_image = Image::from_file("./images/cat.png");

    gtkbox.append(&cat_image);

    window.set_child(gtkbox);
    // Present window
    window.present();
}
