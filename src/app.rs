use adw::prelude::*;
use adw::{Application, ApplicationWindow};
use gtk::{Box, Label, Orientation, Stack, StackSidebar};

pub fn build_ui(app: &Application) {
    // -------------------------------------------------
    // Header
    // -------------------------------------------------

    let header = adw::HeaderBar::builder()
        .title_widget(&Label::new(Some("Tokyo's Brotherhood")))
        .build();

    // -------------------------------------------------
    // Source manager
    // -------------------------------------------------

    let mut source_manager = crate::sources::SourceManager::new();

    source_manager.add_source(
        ::std::boxed::Box::new(
            crate::sources::test_source::TestSource,
        ),
    );

    // Test the source manager.
    let results = source_manager.search_all("One");

    for media in results {
        println!("Found: {}", media.title);
    }

    // -------------------------------------------------
    // Test repository manifest
    // -------------------------------------------------

    let manifest = include_str!("sources/test_repository.json");

    match crate::sources::repository::ExtensionRepository::from_json(manifest) {
        Ok(repository) => {
            println!("Repository: {}", repository.name);

            for extension in repository.extensions {
                println!(
                    "Extension: {} v{}",
                    extension.name,
                    extension.version
                );
            }
        }
        Err(error) => {
            eprintln!("Failed to parse repository: {}", error);
        }
    }

    // -------------------------------------------------
    // Main navigation stack
    // -------------------------------------------------

    let stack = Stack::builder()
        .hexpand(true)
        .vexpand(true)
        .build();

    // -------------------------------------------------
    // Pages
    // -------------------------------------------------

    let home = Label::new(Some("Home"));
    let anime = Label::new(Some("Anime"));
    let manga = Label::new(Some("Manga"));

    // Real Library page
    let library = crate::library::build_library_page();

    // Real History page
    let history = crate::history::build_history_page();

    let downloads = Label::new(Some("Downloads"));

    // Real Extensions page
    let extensions =
        crate::extensions::build_extensions_page(&source_manager);

    let settings = Label::new(Some("Settings"));

    // -------------------------------------------------
    // Navigation order
    // -------------------------------------------------

    stack.add_titled(
        &home,
        Some("home"),
        "Home",
    );

    stack.add_titled(
        &anime,
        Some("anime"),
        "Anime",
    );

    stack.add_titled(
        &manga,
        Some("manga"),
        "Manga",
    );

    stack.add_titled(
        &library,
        Some("library"),
        "Library",
    );

    stack.add_titled(
        &history,
        Some("history"),
        "History",
    );

    stack.add_titled(
        &downloads,
        Some("downloads"),
        "Downloads",
    );

    // -------------------------------------------------
    // Extensions
    // -------------------------------------------------

    let extensions_page = stack.add_titled(
        &extensions,
        Some("extensions"),
        "Extensions",
    );

    extensions_page.set_icon_name("puzzle-piece-symbolic");

    // -------------------------------------------------
    // Settings
    // -------------------------------------------------

    let settings_page = stack.add_titled(
        &settings,
        Some("settings"),
        "Settings",
    );

    settings_page.set_icon_name("settings-symbolic");

    // -------------------------------------------------
    // Sidebar
    // -------------------------------------------------

    let sidebar = StackSidebar::builder()
        .stack(&stack)
        .width_request(220)
        .build();

    // -------------------------------------------------
    // Content area
    // -------------------------------------------------

    let content = Box::new(
        Orientation::Horizontal,
        0,
    );

    content.append(&sidebar);
    content.append(&stack);

    // -------------------------------------------------
    // Root layout
    // -------------------------------------------------

    let root = Box::new(
        Orientation::Vertical,
        0,
    );

    root.append(&header);
    root.append(&content);

    // -------------------------------------------------
    // Application window
    // -------------------------------------------------

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Tokyo's Brotherhood")
        .default_width(1200)
        .default_height(800)
        .content(&root)
        .build();

    window.present();
}