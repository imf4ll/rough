use std::process::Command;
use std::fs::{read_to_string, metadata, File};
use std::io::Write;
use std::env::var;
use std::time::SystemTime;

use reqwest::blocking::Client;
use serde::{Serialize, Deserialize};
use serde_json;

use gtk::prelude::*;
use gtk::{
    ListBox,
    Box,
    Orientation,
    Align,
    Label,
    ScrolledWindow,
    CssProvider,
    Entry
};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Articles {
    articles: Vec<Article>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Article {
    author: String,
    title: String,
    publishedAt: String,
    url: String,
}

pub fn news(
        key: String, region: String, browser: String,
        container: &Box, scrollable: &ScrolledWindow, textbox: &Entry,
        list: &ListBox, provider: &CssProvider, cache_time: u128,
    ) {
    let articles = cache(region, key, cache_time);

    show(&scrollable, &container, &articles.articles, &list, provider, String::new(), &browser);
    
    let scrollable_clone = scrollable.clone();
    let container_clone = container.clone();
    let list_clone = list.clone();
    let provider_clone = provider.clone();
    let browser_clone = browser.clone();

    textbox.connect_changed(move | text | {
        if text.text().as_str() == "" {
            show(&scrollable_clone, &container_clone, &articles.articles, &list_clone, &provider_clone, text.text().to_string(), &browser_clone);

        } 
    });
}

fn show(
        scrollable: &ScrolledWindow, container: &Box, articles: &Vec<Article>,
        list: &ListBox, provider: &CssProvider, text: String, browser: &String,
    ) {
    scrollable.set_height_request(300);

    list.foreach(| i | list.remove(i));
        
    for article in articles.into_iter() {
        let artc = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(5)
            .margin(5)
            .halign(Align::Center)
            .valign(Align::Center)
            .build();

        let title_text = article.title.split(" -").collect::<Vec<&str>>()[0];

        let title = Label::new(Some(if title_text.len() > 100 { &title_text[..100] } else { title_text }));

        title
            .style_context()
            .add_provider(provider, gtk::STYLE_PROVIDER_PRIORITY_USER);

        title
            .style_context()
            .add_class("news-title");
        
        let description = Label::new(Some(&format!("{} · {}", article.author, format_time(&article.publishedAt))));
        
        description
            .style_context()
            .add_provider(provider, gtk::STYLE_PROVIDER_PRIORITY_USER);

        description
            .style_context()
            .add_class("news-description");

        artc.add(&title);
        artc.add(&description);

        list.add(&artc);
    }

    scrollable.add(list);

    container.add(scrollable);
    container.show_all();

    if text == "" {
        let articles_clone = articles.clone();
        let browser_clone = browser.clone();
    
        list.connect_key_press_event(move | list, event | {
            if event.keycode() == Some(36) && list.children().len() == 10  {
                open_article(list, &browser_clone, &articles_clone);

            }
            
            Inhibit(false)
        });

        let articles_clone = articles.clone();
        let browser_clone = browser.clone();
        
        list.connect_button_press_event(move | list, event | {
            if event.button() == 1 && list.children().len() == 10  {
                let articles_clone = articles_clone.clone();
                let browser_clone = browser_clone.clone();

                list.connect_row_selected(move | list, _ | {
                   open_article(list, &browser_clone, &articles_clone);

                });
            }

            Inhibit(false)
        });
    }
}

fn open_article(list: &ListBox, browser: &String, articles: &Vec<Article>) {
    let index = list.selected_row().expect("Failed to get index.").index();

    Command::new(&browser)
        .arg(&articles[index as usize].url)
        .spawn()
        .expect("Failed to open article.");
}

fn format_time(published_at: &String) -> String {
    published_at
        .split("T")
        .collect::<Vec<&str>>()[0]
        .replace("-", "/")
        .to_string()
}

fn cache(region: String, key: String, cache_time: u128) -> Articles {
    let mut path = var("HOME").expect("Failed to get HOME env variable.");

    path.push_str("/.config/rough/news-cache.json");

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Failed to get now duration.")
        .as_millis();

    let md = match metadata(&path) {
        Ok(m) => m,
        Err(_) => return create_cache(region, key, path),
    };

    let time = md
        .modified()
        .expect("Failed to get time duration.")
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Failed to get timestamp.")
        .as_millis();

    if (now - time) / 60000 >= cache_time {
        return create_cache(region, key, path);

    }

    let file = read_to_string(&path)
        .expect("Failed to read file.");

    serde_json::from_str::<Articles>(&file)
        .expect("Failed to serialize.")
}

fn create_cache(region: String, key: String, path: String) -> Articles {
    let data = get(region, key);
    
    let mut file_cache = File::create(&path)
        .expect("Failed to create cache file.");

    let buf = serde_json::to_string(&data)
        .expect("Failed to serialize.");

    file_cache
        .write_all(&buf.as_bytes())
        .expect("Failed to create cache.");

    data
}

fn get(region: String, key: String) -> Articles {
    let client = Client::new();

    client.get(format!("https://newsapi.org/v2/top-headlines?sources=google-news-{}&apiKey={}", region, key))
        .header("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36")
        .send()
        .expect("Failed to get data.")
        .json::<Articles>()
        .expect("Failed to parse JSON.")
}
