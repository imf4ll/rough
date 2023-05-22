mod app;
mod utils;
mod config;
mod styling;
mod tests;
mod modules;

use gtk::Application;
use clap::Parser;
use app::App;

#[derive(Parser)]
#[clap(version, about = "A GTK based app launcher")]
struct Args {
    /// Runs in shell mode
    #[clap(short, long, takes_value = false)]
    shell: bool,

    /// Shows all modules
    #[clap(short, long, takes_value = false)]
    modules: bool,

    /// Enables modules
    #[clap(short, long = "--enable-module", value_name = "MODULE", default_value = "")]
    enablemodule: String,

    /// Disables modules
    #[clap(short, long = "--disable-module", value_name = "MODULE", default_value = "")]
    disablemodule: String,
}

fn main() {
    let args = Args::parse();

    let config = config::parse();
    
    if args.modules {
        modules::show_modules(&config);

        return
    }

    if args.enablemodule != "" {
        args.enablemodule
            .to_lowercase()
            .split(",")
            .collect::<Vec<&str>>()
            .into_iter()
            .for_each(| i | modules::enable_module(i.trim().to_string()));

        return;

    } else if args.disablemodule != "" {
        args.disablemodule
            .to_lowercase()
            .split(",")
            .collect::<Vec<&str>>()
            .into_iter()
            .for_each(| i | modules::disable_module(i.trim().to_string()));

        return;
    }

    let app = App {
        app: Application::builder()
            .application_id("com.z3oxs.rough")
            .build(),

        shell: args.shell,
        config,
    };

    app.run();
}
