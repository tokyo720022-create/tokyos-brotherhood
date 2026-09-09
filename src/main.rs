use adw::gio::prelude::{ApplicationExt, ApplicationExtManual};
use adw::Application;

mod app;
mod extensions;
mod history;
mod library;
mod media;
mod sources;

fn main() {
    let application = Application::builder()
        .application_id("com.tokyo.brotherhood")
        .build();

    application.connect_activate(app::build_ui);

    application.run();
}