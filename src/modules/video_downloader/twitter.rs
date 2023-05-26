use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json;

#[derive(Deserialize, Debug, Clone)]
struct Format {
    content_type: String,
    url: String,
}

#[derive(Debug)]
struct Media {
    formats: Vec<Format>,
    rest_id: String,
}

use crate::modules::video_downloader::types::GlobalFormat;

pub fn get(url: String) -> Vec<GlobalFormat> {
    let token_client = Client::new();
    let tweet_client = Client::new();

    let guest_token_req = token_client
        .post("https://api.twitter.com/1.1/guest/activate.json")
        .header("Authorization", "Bearer AAAAAAAAAAAAAAAAAAAAANRILgAAAAAAnNwIzUejRCOuH5E6I8xnZz4puTs%3D1Zv7ttfk8LF81IUq16cHjhLTvJu4FA33AGWWjCpTnA")
        .send()
        .expect("Failed to request guest token.")
        .text()
        .expect("Failed to get text.");

    let guest_token = guest_token_req
        .split(":\"")
        .collect::<Vec<&str>>()[1]
        .split("\"")
        .collect::<Vec<&str>>()[0];

    let video_id = url.split("/").collect::<Vec<&str>>()[5];

    let tweet_req = tweet_client
        .get(format!("https://twitter.com/i/api/graphql/LJ_TjoWGgNTXCl7gfx4Njw/TweetDetail?variables=%7B%22focalTweetId%22%3A%22{video_id}%22%2C%22with_rux_injections%22%3Afalse%2C%22includePromotedContent%22%3Atrue%2C%22withCommunity%22%3Atrue%2C%22withQuickPromoteEligibilityTweetFields%22%3Atrue%2C%22withBirdwatchNotes%22%3Afalse%2C%22withSuperFollowsUserFields%22%3Atrue%2C%22withDownvotePerspective%22%3Afalse%2C%22withReactionsMetadata%22%3Afalse%2C%22withReactionsPerspective%22%3Afalse%2C%22withSuperFollowsTweetFields%22%3Atrue%2C%22withVoice%22%3Atrue%2C%22withV2Timeline%22%3Afalse%2C%22__fs_dont_mention_me_view_api_enabled%22%3Afalse%2C%22__fs_interactive_text_enabled%22%3Atrue%2C%22__fs_responsive_web_uc_gql_enabled%22%3Afalse%7D"))
        .header("Host", "twitter.com")
        .header("Authorization", "Bearer AAAAAAAAAAAAAAAAAAAAANRILgAAAAAAnNwIzUejRCOuH5E6I8xnZz4puTs%3D1Zv7ttfk8LF81IUq16cHjhLTvJu4FA33AGWWjCpTnA")
        .header("X-Guest-Token", guest_token)
        .send()
        .expect("Failed to request video information")
        .text()
        .expect("Failed to parse information");

    if !tweet_req.contains("\"variants\":") { return vec![] }

    let mut all_medias: Vec<Media> = vec![];

    for i in 0..tweet_req.matches("\"tweet_results\":{").count() + 1 {
        let data = &tweet_req
            .split("\"tweet_results\":").collect::<Vec<&str>>()[i]
            .split(",\"tweetDisplayType").collect::<Vec<&str>>()[0];

        if !data.contains("\"extended_entities\":{") { continue }

        let rest_id = data
            .split("\"rest_id\":\"")
            .collect::<Vec<&str>>()[1]
            .split("\"")
            .collect::<Vec<&str>>()[0];

        let variants = data
            .split("\"variants\":")
            .collect::<Vec<&str>>();

        if variants.len() <= 1 { continue }
            
        let formats = serde_json::from_str::<Vec<Format>>(variants[1].split("}}]},").collect::<Vec<&str>>()[0])
            .expect("Failed to parse JSON.");

        all_medias.push(Media {
            rest_id: rest_id.to_string(),
            formats,
        });
    }
    
    let mut qualities: Vec<GlobalFormat> = vec![];

    for f in all_medias {
        if url.contains(f.rest_id.as_str()) {
            for format in &f.formats {
                let quality_vec: Vec<&str> = format.url.split("/").collect();
                let quality = quality_vec[quality_vec.len() - 2];

                if format.content_type == "video/mp4" {
                    qualities.push(GlobalFormat {
                        quality: quality.to_string(),
                        url: format.url.to_string(),
                        audio: String::from(""),
                    });
                }
            }

            break;
        }
    }

    qualities
}
