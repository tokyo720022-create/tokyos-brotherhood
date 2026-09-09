use gtk::prelude::*;
use gtk::{
    Box, Button, Entry, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow,
};

use crate::media::MediaType;
use crate::sources::SourceManager;

pub fn build_extensions_page(
    source_manager: &SourceManager,
) -> ScrolledWindow {
    let content = Box::new(Orientation::Vertical, 18);

    content.set_margin_top(24);
    content.set_margin_bottom(24);
    content.set_margin_start(24);
    content.set_margin_end(24);

    // -------------------------------------------------
    // Page title
    // -------------------------------------------------

    let title = Label::new(Some("Extensions"));
    title.set_halign(gtk::Align::Start);
    title.add_css_class("title-2");

    content.append(&title);

    // -------------------------------------------------
    // Installed extensions
    // -------------------------------------------------

    let installed_title = Label::new(Some("Installed"));
    installed_title.set_halign(gtk::Align::Start);
    installed_title.add_css_class("title-3");

    content.append(&installed_title);

    let installed_list = ListBox::new();
    installed_list.set_selection_mode(gtk::SelectionMode::None);
    installed_list.add_css_class("boxed-list");

    for source in source_manager.sources() {
        add_extension_row(
            &installed_list,
            source.name(),
            source.media_type(),
        );
    }

    content.append(&installed_list);

    // -------------------------------------------------
    // Add extension button
    // -------------------------------------------------

    let add_button = Button::with_label("Add Extension");
    add_button.set_halign(gtk::Align::Start);

    content.append(&add_button);

    // -------------------------------------------------
    // Extension repositories
    // -------------------------------------------------

    let repositories_title =
        Label::new(Some("Extension Repositories"));

    repositories_title.set_halign(gtk::Align::Start);
    repositories_title.add_css_class("title-3");

    content.append(&repositories_title);

    let repository_list = ListBox::new();
    repository_list.set_selection_mode(gtk::SelectionMode::None);
    repository_list.add_css_class("boxed-list");

    let empty_repository_label =
        Label::new(Some("No extension repositories added."));

    empty_repository_label.set_halign(gtk::Align::Start);
    empty_repository_label.add_css_class("dim-label");

    content.append(&repository_list);
    content.append(&empty_repository_label);

    // -------------------------------------------------
    // Add repository dialog
    // -------------------------------------------------

    let repository_list_clone = repository_list.clone();
    let empty_label_clone = empty_repository_label.clone();

    add_button.connect_clicked(move |button| {
        let Some(window) = button
            .root()
            .and_then(|root| root.downcast::<gtk::Window>().ok())
        else {
            return;
        };

        let dialog = gtk::Dialog::builder()
            .title("Add Extension Repository")
            .transient_for(&window)
            .modal(true)
            .build();

        dialog.add_button(
            "Cancel",
            gtk::ResponseType::Cancel,
        );

        dialog.add_button(
            "Add",
            gtk::ResponseType::Accept,
        );

        let area = dialog.content_area();

        let description = Label::new(Some(
            "Enter the URL of an extension repository.",
        ));

        description.set_halign(gtk::Align::Start);
        description.set_margin_top(12);
        description.set_margin_start(12);
        description.set_margin_end(12);

        area.append(&description);

        let entry = Entry::new();
        entry.set_placeholder_text(Some(
            "https://example.com/extensions.json",
        ));

        entry.set_margin_top(12);
        entry.set_margin_bottom(12);
        entry.set_margin_start(12);
        entry.set_margin_end(12);

        area.append(&entry);

        let repository_list = repository_list_clone.clone();
        let empty_label = empty_label_clone.clone();

        dialog.connect_response(move |dialog, response| {
            if response == gtk::ResponseType::Accept {
                let url = entry.text().trim().to_string();

                if !url.is_empty() {
                    add_repository_row(
                        &repository_list,
                        &empty_label,
                        &url,
                    );
                }
            }

            dialog.close();
        });

        dialog.present();
    });

    // -------------------------------------------------
    // Available extensions
    // -------------------------------------------------

    let available_title = Label::new(Some("Available"));
    available_title.set_halign(gtk::Align::Start);
    available_title.add_css_class("title-3");

    content.append(&available_title);

    let available_label = Label::new(Some(
        "Available extensions will appear after a repository is added.",
    ));

    available_label.set_halign(gtk::Align::Start);
    available_label.add_css_class("dim-label");

    content.append(&available_label);

    // -------------------------------------------------
    // Scroll container
    // -------------------------------------------------

    ScrolledWindow::builder()
        .hexpand(true)
        .vexpand(true)
        .child(&content)
        .build()
}

fn add_extension_row(
    list: &ListBox,
    name: &str,
    media_type: MediaType,
) {
    let row = ListBoxRow::new();

    let container = Box::new(
        Orientation::Horizontal,
        12,
    );

    container.set_margin_top(12);
    container.set_margin_bottom(12);
    container.set_margin_start(16);
    container.set_margin_end(16);

    let info = Box::new(
        Orientation::Vertical,
        4,
    );

    info.set_hexpand(true);

    let name_label = Label::new(Some(name));
    name_label.set_halign(gtk::Align::Start);
    name_label.add_css_class("heading");

    let media_label = match media_type {
        MediaType::Anime => Label::new(Some("Anime")),
        MediaType::Manga => Label::new(Some("Manga")),
    };

    media_label.set_halign(gtk::Align::Start);
    media_label.add_css_class("dim-label");

    info.append(&name_label);
    info.append(&media_label);

    let status_label = Label::new(Some("Enabled"));
    status_label.set_halign(gtk::Align::Center);

    let settings_button =
        Button::with_label("Settings");

    container.append(&info);
    container.append(&status_label);
    container.append(&settings_button);

    row.set_child(Some(&container));
    list.append(&row);
}

fn add_repository_row(
    list: &ListBox,
    empty_label: &Label,
    url: &str,
) {
    empty_label.set_visible(false);

    let row = ListBoxRow::new();

    let container = Box::new(
        Orientation::Horizontal,
        12,
    );

    container.set_margin_top(12);
    container.set_margin_bottom(12);
    container.set_margin_start(16);
    container.set_margin_end(16);

    let url_label = Label::new(Some(url));

    url_label.set_halign(gtk::Align::Start);
    url_label.set_hexpand(true);
    url_label.set_ellipsize(gtk::pango::EllipsizeMode::Middle);

    let remove_button =
        Button::with_label("Remove");

    container.append(&url_label);
    container.append(&remove_button);

    // Remove the repository row.
    let row_clone = row.clone();

    remove_button.connect_clicked(move |_| {
        row_clone.unparent();
    });

    row.set_child(Some(&container));
    list.append(&row);
}