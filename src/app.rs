use adw::prelude::*;
use adw::{Application, ApplicationWindow};
use gtk::{Box, Label, Orientation, Stack, StackSidebar};

pub fn build_ui(app: &Application) {
    let header = adw::HeaderBar::builder()
        .title_widget(&Label::new(Some("Tokyo's Brotherhood")))
        .build();

    let stack = Stack::builder()
        .hexpand(true)
        .vexpand(true)
        .build();

    // Pages
    let home = Label::new(Some("Home"));
    let anime = Label::new(Some("Anime"));
    let manga = Label::new(Some("Manga"));

    // Real Library page
    let library = crate::library::build_library_page();

    let history = Label::new(Some("History"));
    let downloads = Label::new(Some("Downloads"));
    let extensions = Label::new(Some("Extensions"));
    let settings = Label::new(Some("Settings"));

    // Navigation order
    stack.add_titled(&home, Some("home"), "Home");
    stack.add_titled(&anime, Some("anime"), "Anime");
    stack.add_titled(&manga, Some("manga"), "Manga");
    stack.add_titled(&library, Some("library"), "Library");
    stack.add_titled(&history, Some("history"), "History");
    stack.add_titled(&downloads, Some("downloads"), "Downloads");

    // Extensions icon
    let extensions_page =
        stack.add_titled(&extensions, Some("extensions"), "Extensions");
    extensions_page.set_icon_name("puzzle-piece-symbolic");

    // Settings icon
    let settings_page =
        stack.add_titled(&settings, Some("settings"), "Settings");
    settings_page.set_icon_name("settings-symbolic");
    
    // Sidebar
    let sidebar = StackSidebar::builder()
        .stack(&stack)
        .width_request(220)
        .build();

    // Content area
    let content = Box::new(Orientation::Horizontal, 0);
    content.append(&sidebar);
    content.append(&stack);

    // Root layout
    let root = Box::new(Orientation::Vertical, 0);
    root.append(&header);
    root.append(&content);

    // Window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Tokyo's Brotherhood")
        .default_width(1200)
        .default_height(800)
        .content(&root)
        .build();

    window.present();
}