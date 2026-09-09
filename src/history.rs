use gtk::prelude::*;
use gtk::{
    Box, Label, ListBox, ListBoxRow, Orientation, Picture, ScrolledWindow,
};

struct HistoryEntry {
    title: &'static str,
    progress: &'static str,
    media_type: &'static str,
    cover_path: Option<&'static str>,
}

pub fn build_history_page() -> ScrolledWindow {
    let content = Box::new(Orientation::Vertical, 18);

    content.set_margin_top(24);
    content.set_margin_bottom(24);
    content.set_margin_start(24);
    content.set_margin_end(24);

    let title = Label::new(Some("History"));
    title.set_halign(gtk::Align::Start);
    title.add_css_class("title-2");

    content.append(&title);

    // Today
    let today = Label::new(Some("Today"));
    today.set_halign(gtk::Align::Start);
    today.add_css_class("title-3");
    content.append(&today);

    let today_list = ListBox::new();
    today_list.set_selection_mode(gtk::SelectionMode::None);
    today_list.add_css_class("boxed-list");

    let today_entries = [
        HistoryEntry {
            title: "One Piece",
            progress: "Episode 1150",
            media_type: "Anime",
            cover_path: None,
        },
        HistoryEntry {
            title: "Sakamoto Days",
            progress: "Chapter 235",
            media_type: "Manga",
            cover_path: None,
        },
    ];

    for entry in today_entries {
        add_history_entry(&today_list, entry);
    }

    content.append(&today_list);

    // Yesterday
    let yesterday = Label::new(Some("Yesterday"));
    yesterday.set_halign(gtk::Align::Start);
    yesterday.add_css_class("title-3");
    content.append(&yesterday);

    let yesterday_list = ListBox::new();
    yesterday_list.set_selection_mode(gtk::SelectionMode::None);
    yesterday_list.add_css_class("boxed-list");

    let yesterday_entries = [HistoryEntry {
        title: "Frieren",
        progress: "Episode 28",
        media_type: "Anime",
        cover_path: None,
    }];

    for entry in yesterday_entries {
        add_history_entry(&yesterday_list, entry);
    }

    content.append(&yesterday_list);

    ScrolledWindow::builder()
        .hexpand(true)
        .vexpand(true)
        .child(&content)
        .build()
}

fn add_history_entry(list: &ListBox, entry: HistoryEntry) {
    let row = ListBoxRow::new();

    let container = Box::new(Orientation::Horizontal, 14);

    container.set_margin_top(12);
    container.set_margin_bottom(12);
    container.set_margin_start(12);
    container.set_margin_end(12);

    // Cover
    let cover = match entry.cover_path {
        Some(path) => Picture::for_filename(path),
        None => Picture::new(),
    };

    cover.set_size_request(80, 110);
    cover.set_can_shrink(true);

    // Information
    let info = Box::new(Orientation::Vertical, 4);
    info.set_valign(gtk::Align::Center);
    info.set_hexpand(true);

    let title = Label::new(Some(entry.title));
    title.set_halign(gtk::Align::Start);
    title.add_css_class("heading");

    let progress = Label::new(Some(entry.progress));
    progress.set_halign(gtk::Align::Start);

    let media_type = Label::new(Some(entry.media_type));
    media_type.set_halign(gtk::Align::Start);
    media_type.add_css_class("dim-label");

    info.append(&title);
    info.append(&progress);
    info.append(&media_type);

    container.append(&cover);
    container.append(&info);

    row.set_child(Some(&container));
    list.append(&row);
}