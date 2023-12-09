use gtk::prelude::*;

fn build_ui(app: &gtk::Application) {
    let glade_src = include_str!("../layout.glade");
    let builder = gtk::Builder::from_string(glade_src);
    let window: gtk::Window = builder.object(
        "applicationwindow1"
    ).unwrap();
    window.set_application(Some(app));
    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.shinglyu.catsay-gui"),
        Default::default()
    );

    application.connect_activate(build_ui);

    application.run();

}