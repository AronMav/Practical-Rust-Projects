use cursive::views::TextView;

fn main() {
    let mut siv = cursive::default();

    let cat_text = "Meow!
\\
  \\
    /\\_/\\
   ( o o )
   =( I )=";
    // Declaring the app layout
    siv.add_layer(TextView::new(cat_text));
    siv.run();
}
