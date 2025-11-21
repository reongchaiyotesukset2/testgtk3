use gtk::{prelude::*, Builder,Window,Application};
fn main() {
    let application = Application::new(
        Some("com.example.my-basic_ui-app"),
        Default::default(),
    );

    application.connect_activate(move |application|{
    
    let glade_src = include_str!("../window.ui");
    let builder = Builder::from_string(glade_src);

    let window: Window = builder.object("main_window").expect("Couldn't get main_window");
    window.set_application(Some(application));

    window.show_all();
    });
    
    application.run();
}
