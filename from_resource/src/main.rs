use gtk::prelude::*;
use gtk::Builder;

fn build_ui() {
    // The path must be a resource path, e.g., "/com/example/app/window.ui"
    let builder = Builder::from_resource("/path/to/your/ui/file.ui");

    // Get an object from the builder
    let window: gtk::ApplicationWindow = builder
        .get_object("main_window")
        .expect("Couldn't get main_window from builder");

    // Connect signals and show the window
    // ...
}
