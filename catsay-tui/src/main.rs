use cursive::{
    event::Key,
    view::Nameable,
    views::{Checkbox, Dialog, EditView, ListView},
    Cursive, CursiveExt,
};

struct CatsayOptions {
    message: String,
    dead: bool,
}
fn main() {
    let mut siv = Cursive::new();
    input_step(&mut siv);
    siv.add_global_callback(Key::Esc, |s| s.quit());
    siv.add_global_callback('q', |s| s.quit());

    siv.run();
}

fn input_step(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("Please fill out the form for the cat")
            .content(
                ListView::new()
                    .child("Message =>", EditView::new().with_name("message"))
                    .child("dead? =>", Checkbox::new().with_name("dead")),
            )
            .button("OK", |s| {
                let message = s.call_on_name("message", |view: &mut EditView| view.get_content());
                let dead = s.call_on_name("dead", |view: &mut Checkbox| view.is_checked());

                let catsay_opt = CatsayOptions {
                    message: message.unwrap().to_string(),
                    dead: dead.unwrap(),
                };

                // build cat say view with data now.
                make_cat_say(s, catsay_opt);
            }),
    );
}

fn make_cat_say(s: &mut Cursive, options: CatsayOptions) {
    let eye = if options.dead { "x" } else { "❤️" };
    let message = if options.message.to_lowercase() == "woof" {
        "Meow".to_string()
    } else {
        options.message
    };

    let cat_text = format!(
        "{}
    \\
     \\
     /\\___/\\
    ( {} {} )
     =( I )=",
        message, eye, eye
    );

    s.pop_layer();

    s.add_layer(
        Dialog::text(cat_text)
            .title("The cat says:")
            .button("OK", |s| s.quit()),
    );
}
