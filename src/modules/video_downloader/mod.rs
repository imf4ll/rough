mod twitter;
mod facebook;
mod pinterest;
mod reddit;
mod rumble;
mod types;

use std::fs::File;
use std::io::Write;
use std::thread;
use std::sync::{Arc, atomic::{Ordering, AtomicU64}};

use reqwest::blocking::get;
use gtk::prelude::*;
use chrono::prelude::*;

use gtk::{
    Box,
    Entry,
    ListBox,
    Label,
    ScrolledWindow,
    Image,
    Align,
    STYLE_PROVIDER_PRIORITY_USER,
    CssProvider,
};

pub fn video_downloader(
        container: &Box, textbox: &Entry, list: &ListBox,
        scrollable: &ScrolledWindow, path: String, top_bar: &Box,
        provider: &CssProvider,
    ) {
    let container_clone = container.clone();
    let list_clone = list.clone();
    let scrollable_clone = scrollable.clone();
    let path_clone = path.clone();
    let top_bar_clone = top_bar.clone();
    let provider_clone = provider.clone();

    textbox.connect_changed(move | text | {
        let url = text.text().to_string();
        
        let mut filename = String::new();
        let mut qualities: Vec<types::GlobalFormat> = vec![];
        
        if url.contains("twitter.com") {
            qualities = twitter::get(url);

            filename = create_filename("twitter");

            list_clone.set_widget_name("video_downloader");
        
        } else if url.contains("facebook.com") {
            qualities = facebook::get(url);
            
            filename = create_filename("facebook");
 
            list_clone.set_widget_name("video_downloader");

        } else if url.contains("pinterest.com") {
            qualities = pinterest::get(url);
            
            filename = create_filename("pinterest");

            list_clone.set_widget_name("video_downloader");
        
        } else if url.contains("reddit.com") {
            qualities = reddit::get(url);

            filename = create_filename("reddit");

            list_clone.set_widget_name("video_downloader");

        } else if url.contains("rumble.com") {
            qualities = rumble::get(url);

            filename = create_filename("rumble");

            list_clone.set_widget_name("video_downloader");

        }

        if list_clone.widget_name() == "video_downloader" {
            list_clone.foreach(| i | list_clone.remove(i));

            for q in &qualities {
                let item = Box::builder()
                    .halign(Align::Center)
                    .build();

                let quality_label = Label::new(Some(&q.quality));

                item.add(&quality_label);

                list_clone.add(&item);
            }
 
            let size = match list_clone.children().len() as i32 {
                n if n <= 1 => 0,
                n if n <= 2 => 35,
                n if n <= 3 => 65,
                n if n <= 5 => 85,
                _ => 150,
            };

            scrollable_clone.set_height_request(size);

            list_clone.set_halign(Align::Center);

            scrollable_clone.add(&list_clone);

            container_clone.add(&scrollable_clone);
            container_clone.show_all();
        }

        list_clone.connect_key_press_event({
            let qualities = qualities.clone();
            let path = path_clone.clone();
            let filename = filename.clone();
            let container = container_clone.clone();
            let top_bar = top_bar_clone.clone();
            let provider = provider_clone.clone();

            move | list, event | {
                if event.keycode() == Some(36) && list.widget_name() == "video_downloader" {
                    let index = list
                        .selected_row()
                        .expect("Failed to get index.")
                        .index();

                    download(
                        qualities[index as usize].url.clone(),
                        &path,
                        &filename,
                        &container,
                        &top_bar,
                        &provider
                    );
                }
                
                Inhibit(false)        
            }
        });
        
        list_clone.connect_button_press_event({
            let qualities = qualities.clone();
            let path = path_clone.clone();
            let filename = filename.clone();
            let container = container_clone.clone();
            let top_bar = top_bar_clone.clone();
            let provider = provider_clone.clone();
        
            move | list, event | {
                if event.button() == 1 && list.widget_name() == "video_downloader"  {
                    let index = list
                        .selected_row()
                        .expect("Failed to get index.")
                        .index();

                    download(
                        qualities[index as usize].url.clone(),
                        &path,
                        &filename,
                        &container,
                        &top_bar,
                        &provider
                    );
                }

                Inhibit(false)
            }
        });
    });
}

fn download(url: String, path: &String, filename: &String, container: &Box, top_bar: &Box, provider: &CssProvider) {
    container.set_height_request(0);

    let video_bytes = get(url)
        .expect("Failed to download video.");

    let video_size = video_bytes
        .content_length()
        .expect("Failed to get content length.");

    let mut file = File::create(format!("{}/{}", path, filename))
        .expect("Failed to create file.");

    let file_size = Arc::new(AtomicU64::new(0));

    thread::spawn({
        let file_size = Arc::clone(&file_size);

        move || {
            file.write_all(&video_bytes.bytes().expect("Failed to download video."))
                .expect("Failed to write file.");
        
            file_size.store(file.metadata().expect("Failed to get metadata").len(), Ordering::Release);
        }
    });

    while file_size.load(Ordering::Acquire) <= video_size {
        let file_size = file_size.load(Ordering::Acquire);
        
        if file_size == video_size {
            break;

        }
    }

    let success = Image::builder()
        .icon_name("document-save")
        .build();

    success
        .style_context()
        .add_provider(provider, STYLE_PROVIDER_PRIORITY_USER);

    success
        .style_context()
        .add_class("download");

    top_bar.add(&success);
    
    top_bar.reorder_child(&top_bar.children()[top_bar.children().len() - 1], 0);

    top_bar.show_all();
}

fn create_filename(platform: &'static str) -> String {
    format!("{platform}-{}.mp4", Local::now().format("%H_%M_%S-%Y_%m_%d"))

}
