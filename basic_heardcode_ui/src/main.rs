use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, ListBox, ListBoxRow, Label};
/*style 1*/
/*
fn main() {
    let application = Application::new(
        Some("com.example.basic_heardcode_ui"),
        Default::default(),
    );

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("ListBox Example");
        window.set_default_size(300, 200);

        let list_box = ListBox::new();
        list_box.set_selection_mode(gtk::SelectionMode::Single);

        // Add some rows
        for i in 0..5 {
            let row = ListBoxRow::new();
            let label = Label::new(Some(&format!("Item {}", i)));
            row.add(&label);
            list_box.add(&row);
        }

        window.add(&list_box);
        window.show_all();
    });
 
    application.run();
}
*/
/*style 2*/
fn main() {
    let application = Application::new(
        Some("com.example.basic_heardcode_ui"),
        Default::default(),
    );

   application.connect_activate(|app| {
        build_ui(app);
    });
     application.run();
}
    
   fn build_ui(application: &Application) 
    {
            let window = ApplicationWindow::new(application);
        window.set_title("ListBox Example");
        window.set_default_size(300, 200);

        let list_box = ListBox::new();
        list_box.set_selection_mode(gtk::SelectionMode::Single);

        // Add some rows
        for i in 0..5 {
            let row = ListBoxRow::new();
            let label = Label::new(Some(&format!("Item {}", i)));
            row.add(&label);
            list_box.add(&row);
        }

        window.add(&list_box);
        window.show_all();
    }
 
   
    

