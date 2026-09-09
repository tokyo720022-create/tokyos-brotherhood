use adw::Application;
use adw::gio::prelude::{ApplicationExt, ApplicationExtManual};

mod app;

fn main() {
    let application = Application::builder()
        .application_id("com.tokyo.brotherhood")
        .build();

    application.connect_activate(app::build_ui);

    application.run();
}