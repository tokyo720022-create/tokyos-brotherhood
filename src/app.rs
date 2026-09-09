use adw::prelude::*;
use adw::{Application, ApplicationWindow};

pub fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Tokyo's Brotherhood")
        .default_width(1200)
        .default_height(800)
        .build();

    window.present();
}