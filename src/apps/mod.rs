use std::fs::{read_dir, read_to_string};

use crate::utils::types::App;

pub fn get(path: &str) -> Vec<App> {
    let mut apps: Vec<App> = vec![];

    for file in read_dir(path).expect("Failed to read directory") {
        let mut app = App {
            name: String::from(""),
            generic: String::from(""),
            exec: String::from(""),
            icon: String::from(""),
        };

        let file_path = file
            .expect("Failed to read file")
            .path();

        let content = read_to_string(file_path)
            .expect("Failed to read file");

        let raw_lines = content
            .split("\n")
            .collect::<Vec<&str>>();

        for i in &raw_lines {
            if i.contains("Desktop Action") { break }

            if i.starts_with("Name=") {
                app.name = i
                    .split("=")
                    .collect::<Vec<&str>>()[1]
                    .to_string();
            }

            if i.starts_with("GenericName=") {
                app.generic = i
                    .split("=")
                    .collect::<Vec<&str>>()[1]
                    .to_string();
            }

            if i.starts_with("Exec=") {
                app.exec = i
                    .split("=")
                    .collect::<Vec<&str>>()[1]
                    .to_string();
            }

            if i.starts_with("Icon=") {
                app.icon = i
                    .split("=")
                    .collect::<Vec<&str>>()[1]
                    .to_string();
            }
        }
        
        apps.push(app);
    }

    apps
}
