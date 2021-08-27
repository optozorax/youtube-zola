use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct YoutubeMeta {
    upload_date: String,
    fulltitle: String,
    duration: u64,
    channel_url: String,
    thumbnail: String,
    channel: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct YoutubeMetaOut {
    date: String,
    title: String,
    minutes: u64,
    seconds: u64,
    channel_url: String,
    channel_name: String,
    image_path: String,
}

fn main() {
    let id = std::env::args().nth(1).unwrap();
    let json = Command::new("youtube-dl")
        .args(&[
            "-j",
            "--skip-download",
            &format!("https://www.youtube.com/watch?v={}", id),
        ])
        .output()
        .unwrap();
    let mut meta: YoutubeMeta = serde_json::from_slice(&json.stdout).unwrap();
    let thumbnail = reqwest::blocking::get(meta.thumbnail).unwrap();

    let format = match thumbnail
        .headers()
        .get("content-type")
        .unwrap()
        .to_str()
        .unwrap()
    {
        "image/webp" => "webp",
        "image/jpeg" => "jpeg",
        "image/jpg" => "jpg",
        "image/png" => "png",
        _ => unreachable!(),
    };
    let image_path = format!("static/youtube/{}.{}", id, format);
    std::fs::write(&image_path, thumbnail.bytes().unwrap()).unwrap();
    meta.upload_date = format!(
        "{}-{}-{}",
        &meta.upload_date[0..4],
        &meta.upload_date[4..6],
        &meta.upload_date[6..8]
    );

    let out = YoutubeMetaOut {
        date: meta.upload_date,
        title: meta.fulltitle,
        minutes: meta.duration / 60,
        seconds: meta.duration % 60,
        channel_url: meta.channel_url,
        channel_name: meta.channel,
        image_path: format!("youtube/{}.{}", id, format),
    };

    std::fs::write(
        &format!("static/youtube/{}.json", id),
        serde_json::to_string(&out).unwrap(),
    )
    .unwrap();
}
