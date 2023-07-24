use std::process::Command;
use std::fs::{read_to_string, metadata, File};
use std::io::Write;
use std::env::var;
use std::time::SystemTime;
use std::str;

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
#[serde(rename_all = "camelCase")]
struct Article {
    author: String,
    title: String,
    published_at: String,
    url: String,
}

pub fn news(
        key: String, region: String, browser: String,
        container: &Box, scrollable: &ScrolledWindow, textbox: &Entry,
        list: &ListBox, provider: &CssProvider, cache_time: u128,
    ) {
    let articles = cache(region, key, cache_time);

    show(&scrollable, &container, &articles.articles, &list, provider, String::new(), &browser);

    textbox.connect_changed({
        let scrollable = scrollable.clone();
        let container = container.clone();
        let list = list.clone();
        let provider = provider.clone();
        let browser = browser.clone();

        move | text | {
            if text.text().as_str() == "" {
                show(&scrollable, &container, &articles.articles, &list, &provider, text.text().to_string(), &browser);

            } 
        }
    });
}

fn show(
        scrollable: &ScrolledWindow, container: &Box, articles: &Vec<Article>,
        list: &ListBox, provider: &CssProvider, text: String, browser: &String,
    ) {
    scrollable.set_height_request(300);

    list.set_widget_name("news");

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

        let title = Label::new(Some(title_text));

        if title_text.len() >= 90 {
            let mut text = String::new();

            title_text.split("").into_iter().for_each(| i |
                if text.len() < 90 {
                    text.push_str(i);

                }
            );

            title.set_label(&format!("{text}..."));
        }

        title
            .style_context()
            .add_provider(provider, gtk::STYLE_PROVIDER_PRIORITY_USER);

        title
            .style_context()
            .add_class("news-title");
        
        let description = Label::new(Some(&format!("{} · {}", article.author, format_time(&article.published_at))));
        
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
        list.connect_key_press_event({
            let articles = articles.clone();
            let browser = browser.clone();

            move | list, event | {
                if event.keycode() == Some(36) && list.widget_name() == "news" {
                    open_article(list, &browser, &articles);

                }
                
                Inhibit(false)
            }
        });
        
        list.connect_button_press_event({
            let articles = articles.clone();
            let browser = browser.clone();  

            move | list, event | {
                if event.button() == 1 && list.widget_name() == "news"  {
                    list.connect_row_selected({
                        let articles = articles.clone();
                        let browser = browser.clone();
                        
                        move | list, _ | {
                            open_article(list, &browser, &articles);

                        }
                    });
                }

                Inhibit(false)
            }
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

    if (now - time) / 120000 >= cache_time {
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
