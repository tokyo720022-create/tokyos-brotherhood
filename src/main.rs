use adw::prelude::*;
use adw::{Application, ApplicationWindow};

fn main() {
    let app = Application::builder()
        .application_id("com.tokyo.brotherhood")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Anime & Manga")
            .default_width(1000)
            .default_height(700)
            .build();

        window.present();
    });

    app.run();
}