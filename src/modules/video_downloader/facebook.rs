use crate::modules::video_downloader::types::GlobalFormat;
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json;

#[derive(Debug, Deserialize)]
struct TempFormat {
    base_url: String,
    width: u32,
    height: u32,
}

#[derive(Debug, Deserialize)]
pub struct StandardFormat {
    playable_url: String,
    playable_url_quality_hd: String,
}

pub fn get(url: String) -> Vec<GlobalFormat> {
    let client = Client::new();

    let req = client
        .get(url)
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4844.51 Safari/537.36")
        .send()
        .expect("Failed to request video information")
        .text()
        .expect("Failed to parse information");

    let mut qualities: Vec<GlobalFormat> = vec![];
    
    if req.contains("\"representations\":") {
        let formats: Vec<TempFormat> = serde_json::from_str(
            &req
                .split("\"representations\":").collect::<Vec<&str>>()[1]
                .split(",\"video_id\"").collect::<Vec<&str>>()[0]
        ).unwrap();

        let formats_with_audio: StandardFormat = serde_json::from_str(
            &format!("{{{}}}", &req
                .split("init\":null,").collect::<Vec<&str>>()[1]
                .split(",\"spherical_").collect::<Vec<&str>>()[0])
        ).unwrap();

        qualities.push(GlobalFormat {
            quality: String::from("SD w/ audio"),
            url: formats_with_audio.playable_url,
            audio: String::from(""),
        });
        
        qualities.push(GlobalFormat {
            quality: String::from("HD w/ audio"),
            url: formats_with_audio.playable_url_quality_hd,
            audio: String::from(""),
        });

        for format in formats {
            if format.width <= 0 || format.height <= 0 { continue }

            qualities.push(GlobalFormat {
                url: format.base_url,
                quality: format!("{}x{}", format.width, format.height),
                audio: String::from(""),
            });
        }

    } else if req.contains("\"playable_url\":") {
        let format_sd = &req
            .split("\"playable_url\":\"").collect::<Vec<&str>>()[1]
            .split("\",\"playable_url").collect::<Vec<&str>>()[0];

        qualities.push(GlobalFormat {
            url: format_sd.to_string().replace("\\", ""),
            quality: String::from("SD"),
            audio: String::from(""),
        });

        let format_hd = &req
            .split("\"playable_url_quality_hd\":\"").collect::<Vec<&str>>()[1]
            .split("\",\"spherical").collect::<Vec<&str>>()[0];

        qualities.push(GlobalFormat {
            url: format_hd.to_string().replace("\\", ""),
            quality: String::from("HD"),
            audio: String::from(""),
        });

    } else {
        return vec![];

    }

    qualities
}
