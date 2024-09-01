use leptos::logging::log;
use reqwest::Client;
use serde_json::{json, Value};
use shared_lib::shared::recently_played::RecentlyPlayed;
use std::error::Error;

pub async fn play(spotify_uri: &str, device_id: &str, access_token: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url = format!("https://api.spotify.com/v1/me/player/play?device_id={}", device_id);
    
    let mut request = client.put(&url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", access_token));
    
    if spotify_uri.is_empty() {
        request = request.body("{}");
    } else {
        let body = json!({ "uris": [spotify_uri] });
        request = request.body(body.to_string());
    }
    
    let response = request.send().await?;
    
    if response.status().is_success() {
        println!("Playback started");
    } else {
        eprintln!("Playback failed");
    }
    
    Ok(())
}

pub async fn pause(device_id: &str, access_token: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url = format!("https://api.spotify.com/v1/me/player/pause?device_id={}", device_id);
    
    let response = client.put(&url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;
    
    if response.status().is_success() {
        println!("Playback paused");
    } else {
        eprintln!("Playback failed");
    }
    
    Ok(())
}

pub async fn next(device_id: &str, access_token: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url = format!("https://api.spotify.com/v1/me/player/next?device_id={}", device_id);
    
    let response = client.post(&url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;
    
    if response.status().is_success() {
        println!("Playback started");
    } else {
        eprintln!("Playback failed");
    }
    
    Ok(())
}

pub async fn previous(device_id: &str, access_token: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url = format!("https://api.spotify.com/v1/me/player/previous?device_id={}", device_id);
    
    let response = client.post(&url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;
    
    if response.status().is_success() {
        println!("Playback started");
    } else {
        eprintln!("Playback failed");
    }
    
    Ok(())
}

pub async fn shuffle(device_id: &str, access_token: &str, shuffle: bool) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url = format!(
        "https://api.spotify.com/v1/me/player/shuffle?device_id={}&state={}",
        device_id, shuffle
    );
    
    let response = client.put(&url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;
    
    if response.status().is_success() {
        println!("Playback started");
    } else {
        eprintln!("Playback failed");
    }
    
    Ok(())
}

pub async fn repeat(device_id: &str, access_token: &str, state: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url = format!(
        "https://api.spotify.com/v1/me/player/repeat?device_id={}&state={}",
        device_id, state
    );
    
    let response = client.put(&url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;
    
    if response.status().is_success() {
        println!("Playback started");
    } else {
        eprintln!("Playback failed");
    }
    
    Ok(())
}

pub async fn volume(device_id: &str, access_token: &str, volume: u8) -> Result<(), Box<dyn Error>> {
    let volume = volume.min(100).max(0);
    let client = Client::new();
    let url = format!(
        "https://api.spotify.com/v1/me/player/volume?device_id={}&volume_percent={}",
        device_id, volume
    );
    
    let response = client.put(&url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;
    
    if response.status().is_success() {
        println!("Playback started");
    } else {
        eprintln!("Playback failed");
    }
    
    Ok(())
}

pub async fn get_last_played_track(token: &str) -> Option<RecentlyPlayed> {
    let client = Client::new();
    let res = client
        .get("https://api.spotify.com/v1/me/player/recently-played")
        .bearer_auth(token)
        .send()
        .await
        .ok()?;

    let body = res.text().await.ok()?;
    let json: Value = serde_json::from_str(&body).ok()?;

    if let Some(items) = json["items"].as_array() {
        if let Some(last_played) = items.first() {
            let track = &last_played["track"];
            let track_name = track["name"].as_str().unwrap_or("Unknown");
            let artists = track["artists"]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .map(|artist| artist["name"].as_str().unwrap_or("Unknown"))
                .collect::<Vec<&str>>()
                .join(", ");
            let duration_ms = track["duration_ms"].as_i64().unwrap_or(0);
            let image_url = track["album"]["images"]
                .as_array()
                .and_then(|images| images.last())
                .and_then(|image| image["url"].as_str())
                .unwrap_or("No image");

            return Some(RecentlyPlayed {
                track_name: track_name.to_string(),
                artists: artists.to_string(),
                duration_ms: duration_ms as i32,
                image_url: image_url.to_string(),
            });
        }
    }

    None
}