mod app;
mod utils;
mod config;
mod styling;
mod tests;

use gtk::Application;
use clap::Parser;
use app::App;

#[derive(Parser)]
#[clap(version, about = "A GTK based app launcher")]
struct Args {
    /// Run in shell mode
    #[clap(short, long, takes_value = false)]
    shell: bool,

    /// Calculator mode
    #[clap(short, long, action)]
    calc: bool,
}

fn main() {
    let args = Args::parse();

    let config = config::parse();

    let app = App {
        app: Application::builder()
            .application_id("com.z3oxs.rough")
            .build(),

        shell: args.shell,
        config,
        calc: args.calc,
    };

    app.run();
}
