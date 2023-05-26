use crate::modules::video_downloader::types::GlobalFormat;
use reqwest::blocking::Client;

const QUALITIES: [u32; 5] = [240, 360, 480, 720, 1080];

pub fn get(url: String) -> Vec<GlobalFormat> {
    let client = Client::new();

    let res = client
        .get(format!("{url}.json"))
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36")
        .send()
        .expect("Failed to request video information")
        .text()
        .expect("Failed to parse video information");

    if !res.contains("\"url_overridden_by_dest\"") {
        return vec![];

    }

    let video_id = res
        .split("\"url_overridden_by_dest\": \"").collect::<Vec<&str>>()[1]
        .split("\"").collect::<Vec<&str>>()[0]
        .split("/").collect::<Vec<&str>>()[3];

    let quality: u32 = res
        .split("\"fallback_url\": \"").collect::<Vec<&str>>()[1]
        .split("\"").collect::<Vec<&str>>()[0]
        .split("DASH_").collect::<Vec<&str>>()[1]
        .split(".").collect::<Vec<&str>>()[0]
        .parse()
        .expect("Failed to parse quality label");

    let mut qualities: Vec<GlobalFormat> = vec![];

    for q in QUALITIES {
        if q <= quality {
            qualities.push(GlobalFormat {
                url: format!("https://v.redd.it/{video_id}/DASH_{q}.mp4"),
                quality: format!("{q}p"),
                audio: format!("https://v.redd.it/{video_id}/DASH_audio.mp4"),
            });
        }
    }

    qualities
}
