mod app;
mod utils;
mod config;

use gtk::prelude::*;
use gtk::Application;
use app::handler;
use clap::Parser;

#[derive(Parser)]
#[clap(version, about = "A GTK based app launcher")]
struct Args {
    /// Run in shell mode
    #[clap(short, long, takes_value = false)]
    shell: bool
}

fn main() {
    let args = Args::parse();

    let config = config::parse();
   
    let app = Application::builder()
        .application_id("com.z3oxs.rough")
        .build();

    handler(&app, args.shell, config);

    app.run_with_args(&[""]);
}
