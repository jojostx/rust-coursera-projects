use gtk::{Application, ApplicationWindow, BoxLayout, Button, Image, glib};
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
        .default_width(1000)
        .default_height(500)
        .build();

    let gtkbox = GtkBox::new(Orientation::Vertical, 10);
    let layout = BoxLayout::new(Orientation::Vertical);

    gtkbox.set_layout_manager(Some(layout));

    let markup = r#"<span font_desc="Sans 24">My name is Lulu and my mum's name is Mimi
                                    \
                                     \</span>"#;
    let label = Label::new(None);

    label.set_markup(markup);

    gtkbox.append(&label);

    let cat_image = Image::from_file("./images/cat.png");

    cat_image.set_hexpand(true);
    cat_image.set_vexpand(true);

    gtkbox.append(&cat_image);

    let button = add_btn("Press Me!");

    // Connect to "clicked" signal of `button`
    button.connect_clicked(|button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    gtkbox.append(&button);

    window.set_child(Some(&gtkbox));

    // Present window
    window.present();
}

fn add_btn(label: &str) -> Button {
    let button = Button::builder()
        .label(label)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button
}
