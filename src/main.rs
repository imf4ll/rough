mod app_use;

use gtk::prelude::*;
use gtk::Application;
use app_use::handler;

fn main() {
    let app = Application::builder()
        .application_id("com.z3oxs.rough")
        .build();

    handler(&app);

    app.run();
}
