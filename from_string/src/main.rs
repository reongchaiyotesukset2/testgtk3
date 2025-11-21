use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Builder};

fn build_ui(application: &Application) {
    // 1. Define your GTK UI in an XML string.
    // Often, you might load this using include_str! from a separate file.
    let ui_src = r#"
    <?xml version="1.0" encoding="UTF-8"?>
    <interface>
      <object class="GtkApplicationWindow" id="main_window">
        <property name="application">app</property>
        <property name="default_width">320</property>
        <property name="default_height">200</property>
        <property name="title">GTK from String Example</property>
      </object>
    </interface>
    "#;

    // 2. Create the builder from the string source.
    let builder = Builder::from_string(ui_src);

    // 3. Retrieve objects from the builder by their names (IDs).
    // The `get_object` method is used for this purpose.
    let window: ApplicationWindow = builder
        .get_object("main_window")
        .expect("Couldn't get 'main_window' object");
    
    // 4. Set the application (if not already set in XML) and show the window.
    // The application property was set in the XML in this example.
    window.show_all();
}

fn main() {
    let app = Application::builder()
        .application_id("org.example.from_string_grk3_rs")
        .build();

    app.connect_activate(build_ui);
    app.run();
}
