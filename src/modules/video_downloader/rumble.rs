use crate::modules::video_downloader::types::GlobalFormat;
use reqwest::blocking;
use serde::Deserialize;
use serde_json;

#[derive(Debug, Deserialize)]
struct TempFormat {
    url: String,
    meta: TempFormatMeta,
}

#[derive(Debug, Deserialize)]
struct TempFormatMeta {
    w: u32,
    h: u32,
}

pub fn get(url: String) -> Vec<GlobalFormat> {
    let video_id_req = blocking::get(url)
        .expect("Failed to request video ID")
        .text()
        .expect("Failed to parse video ID information");

    let video_id = video_id_req
        .split("\"video\":\"").collect::<Vec<&str>>()[1]
        .split("\"").collect::<Vec<&str>>()[0];

    let video_response = blocking::get(
        format!(
            "https://rumble.com/embedJS/u3/?request=video&ver=2&v={}&ext=%7B%22ad_count%22%3Anull%7D&ad_wt=9043"
            , video_id
        )
    )
        .expect("Failed to get video information")
        .text()
        .expect("Failed to parse video information");

    if !video_response.contains("") {
        return vec![];

    }

    let raw_formats = video_response
        .split("\"ua\":{\"mp4\":{").collect::<Vec<&str>>()[1]
        .split("}},\"i\"").collect::<Vec<&str>>()[0];

    let formats: Vec<TempFormat> = serde_json::from_str(
        &format!("[{}]", raw_formats
            .replace("\"240\":", "")
            .replace("\"360\":", "")
            .replace("\"480\":", "")
            .replace("\"720\":", "")
            .replace("\"1080\":", "")
        )
    ).unwrap();

    let mut qualities: Vec<GlobalFormat> = vec![];

    for format in formats {
        qualities.push(GlobalFormat {
            url: format.url,
            quality: format!("{}x{}", format.meta.w, format.meta.h),
            audio: String::from(""),
        })
    }

    qualities
}
