use cursive::{
    views::TextView,
    event::Key};

fn main() {
    let mut siv = cursive::default();

    let cat_text = "Meow!
\\
  \\
    /\\_/\\
   ( o o )
   =( I )=";
    // Declaring the app layer
    siv.add_layer(TextView::new(cat_text));

    // Listen to Key::Esk and quit
    siv.add_global_callback(Key::Esc, |s| s.quit());

    siv.run();
}
