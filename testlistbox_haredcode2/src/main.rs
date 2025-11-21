use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, ListBox, ListBoxRow, Label};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // Create a GTK application
    let application = Application::builder()
        .application_id("com.example.listbox_app")
        .build();

    // Connect to the "activate" signal to show the window and set up the UI
    application.connect_activate(|app| {
        build_ui(app);
    });

    // Run the application
    application.run();
}

fn build_ui(application: &Application) {
    let window = ApplicationWindow::builder()
        .application(application)
        .title("ListBox Example")
        .default_width(300)
        .default_height(250)
        .build();

    // Create the ListBox
    let list_box = ListBox::new();
    list_box.set_selection_mode(gtk::SelectionMode::Single); // Set to single selection mode

    // Add some items to the listbox
    let items = vec!["Apple", "Banana", "Blueberry", "Grape", "Pineapple"];
    for item_text in items {
        let label = Label::new(Some(item_text));
        let list_box_row = ListBoxRow::new();
        list_box_row.set_child(Some(&label));
        list_box.add(&list_box_row);
    }

    // Wrap the list box in Rc<RefCell<_>> to share it in the closure if needed
    // In this simple case, we just need a reference when the signal is triggered
    let list_box_rc = Rc::new(RefCell::new(list_box));
    
    // Set up the event handler (signal) for "row-activated"
    // This runs when a user double-clicks or presses Enter on a selected row
    list_box_rc.borrow().connect_row_activated(|list_box, row| {
        // Retrieve the index of the activated row
        let index = row.index();
        
        // In a real app, you would use the index or retrieve data associated with the row
        println!("Row activated: index {}", index);
        
        // Example of retrieving the label text from the row's child widget
        if let Some(child_widget) = row.child() {
            if let Ok(label) = child_widget.downcast::<Label>() {
                println!("Selected item text: {}", label.text());
            }
        }
    });

    // Add the listbox to the window
    window.set_child(Some(&*list_box_rc.borrow()));
    window.show_all();
}
