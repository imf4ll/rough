use crate::modules::video_downloader::types::GlobalFormat;
use reqwest::blocking;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Format {
    url: String,
    width: u32,
    height: u32,
}

pub fn get(url: String) -> Vec<GlobalFormat> {
    let res = blocking::get(url)
        .expect("Failed to request video information")
        .text()
        .expect("Failed to parse video information");

    let quality: Format = serde_json::from_str(
        format!("{}}}",
            &res
                .split("\"V_720P\":")
                .collect::<Vec<&str>>()[1]
                .split("},\"")
                .collect::<Vec<&str>>()[0]
        )
            .as_str()
    )
        .expect("Failed to parse JSON");

    let mut qualities: Vec<GlobalFormat> = vec![];
    
    qualities.push(GlobalFormat {
        url: quality.url,
        quality: format!("{}x{}", quality.width, quality.height),
        audio: String::from(""),
    });

    qualities
}
