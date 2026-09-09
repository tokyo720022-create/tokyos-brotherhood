use adw::prelude::*;
use gtk::{
    Box, Button, Dialog, Entry, Label, ListBox, ListBoxRow, Orientation, ResponseType,
};

pub fn build_library_page() -> Box {
    let root = Box::new(Orientation::Vertical, 18);
    root.set_margin_top(24);
    root.set_margin_bottom(24);
    root.set_margin_start(24);
    root.set_margin_end(24);

    // Page title
    let title = Label::new(Some("Library"));
    title.set_halign(gtk::Align::Start);
    title.add_css_class("title-2");

    root.append(&title);

    // Library filters
    let filters = Box::new(Orientation::Horizontal, 8);

    for name in ["All", "Watching", "Reading", "Completed"] {
        let button = Button::with_label(name);
        filters.append(&button);
    }

    root.append(&filters);

    // Category header
    let category_header = Box::new(Orientation::Horizontal, 8);

    let category_title = Label::new(Some("Categories"));
    category_title.set_halign(gtk::Align::Start);
    category_title.set_hexpand(true);
    category_title.add_css_class("title-3");

    let add_button = Button::with_label("Add Category");

    category_header.append(&category_title);
    category_header.append(&add_button);

    root.append(&category_header);

    // Category list
    let category_list = ListBox::new();
    category_list.set_selection_mode(gtk::SelectionMode::None);
    category_list.add_css_class("boxed-list");

    for category in ["Favorites", "Isekai", "To Translate"] {
        add_category_row(&category_list, category);
    }

    root.append(&category_list);

    // Add category dialog
    add_button.connect_clicked(move |button| {
        let Some(window) = button
            .root()
            .and_then(|root| root.downcast::<gtk::Window>().ok())
        else {
            return;
        };

        let dialog = Dialog::builder()
            .title("Add Category")
            .transient_for(&window)
            .modal(true)
            .build();

        dialog.add_button("Cancel", ResponseType::Cancel);
        dialog.add_button("Add", ResponseType::Accept);

        let content = dialog.content_area();

        let entry = Entry::new();
        entry.set_placeholder_text(Some("Category name"));
        entry.set_margin_top(12);
        entry.set_margin_bottom(12);
        entry.set_margin_start(12);
        entry.set_margin_end(12);

        content.append(&entry);

        // Find the category list from the dialog's parent window.
        let category_list = category_list.clone();

        dialog.connect_response(move |dialog, response| {
            if response == ResponseType::Accept {
                let name = entry.text().trim().to_string();

                if !name.is_empty() {
                    add_category_row(&category_list, &name);
                }
            }

            dialog.close();
        });

        dialog.present();
    });

    root
}

fn add_category_row(list: &ListBox, name: &str) {
    let row = ListBoxRow::new();

    let content = Box::new(Orientation::Horizontal, 12);
    content.set_margin_top(10);
    content.set_margin_bottom(10);
    content.set_margin_start(12);
    content.set_margin_end(12);

    let label = Label::new(Some(name));
    label.set_halign(gtk::Align::Start);
    label.set_hexpand(true);

    let edit_button = Button::with_label("Edit");
    let delete_button = Button::with_label("Delete");

    content.append(&label);
    content.append(&edit_button);
    content.append(&delete_button);

    row.set_child(Some(&content));
    list.append(&row);
}