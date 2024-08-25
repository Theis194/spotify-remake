use async_std::task::sleep;
use std::time::Duration;
use chrono::Utc;
use shared_lib::shared::{
    profile_data::ProfileData, 
    spotify_objects::{top_artists::TopArtists, top_tracks::TopTracks, user::SpotifyUser}
};

use crate::util::{
    config::{
        Config, 
        Value
    }, 
    spotify::{
        auth::refresh_auth_token, 
        util::request
    }, 
    spotify_bb_error::BbError
};

use super::auth::is_user_authorized;

#[tauri::command]
pub fn get_user_profile(filename: String) -> Result<SpotifyUser, BbError> {
    let config = Config::new()
        .set_filename(filename)
        .read()
        .expect("Failed to read config");

    let user_profile = match config.get("user_profile") {
        Some(user_profile) => {
            Some(user_profile.get_spotify_user().unwrap())
        }
        None => {
            None
        }
    };

    match user_profile {
        Some(user_profile) => Ok(user_profile.clone()),
        None => Err(BbError::NoUserProfileError)
    }
}

#[tauri::command]
pub async fn get_profile_data() -> Result<ProfileData, BbError> {
    let mut config = Config::new()
        .set_filename("cache".to_string())
        .read()
        .expect("Failed to read config");

    // Check if the auth token has expired
    let token_expires = *match config.get("auth_token_expires") {
        Some(token_expires) => {
            Some(token_expires.get_date().unwrap())
        }
        None => {
            None
        }
    }.expect("No token expires found");

    let now = Utc::now();

    if now > token_expires {
        refresh_auth_token().await.expect("Error refreshing auth token");
    }

    // Get the auth token
    let auth_token = match config.get("auth_token") {
        Some(auth_token) => {
            Some(auth_token.get_auth_response().unwrap().access_token.clone())
        }
        None => {
            None
        }
    }.expect("No auth token found");

    let client = reqwest::Client::new();

    // Get the user profile
    let user_profile = match config.get("user_profile") {
        Some(user_profile) => {
            Some(user_profile.get_spotify_user().unwrap())
        }
        None => {
            None
        }
    }.expect("No user profile found").clone();

    // Get the last request dates
    let last_request_top_tracks = *match config.get("last_request_top_tracks") {
        Some(last_request_top_tracks) => {
            if last_request_top_tracks.matches(&Value::Date(Utc::now())) {
                Some(last_request_top_tracks.get_date().unwrap())
            } else {
                None
            }
        }
        None => {
            None
        }
    }.expect("No last request top tracks found");

    let last_request_top_artists = *match config.get("last_request_top_artists") {
        Some(last_request_top_artists) => {
            if last_request_top_artists.matches(&Value::Date(Utc::now())) {
                Some(last_request_top_artists.get_date().unwrap())
            } else {
                None
            }
        }
        None => {
            None
        }
    }.expect("No last request top artists found");

    // Check if the top tracks are still valid
    let top_tracks = if now - last_request_top_tracks > chrono::Duration::days(1) {
        let top_tracks = request::<TopTracks>("https://api.spotify.com/v1/me/top/tracks?time_range=medium_term".to_string(), &auth_token, &client).await.expect("Error getting top tracks");
        config.set("top_tracks".to_string(), Value::TopTracks(top_tracks.clone()))
            .write().expect("Error writing config");

        top_tracks
    } else {
        match config.get("top_tracks") {
            Some(top_tracks) => {
                top_tracks.get_top_tracks().expect("Error getting top tracks").clone()
            }
            None => {
                let top_tracks = request::<TopTracks>("https://api.spotify.com/v1/me/top/tracks?time_range=medium_term".to_string(), &auth_token, &client).await.expect("Error getting top tracks");
                config.set("top_tracks".to_string(), Value::TopTracks(top_tracks.clone()))
                    .write().expect("Error writing config");

                top_tracks
            }
        }
    };
    
    // Check if the top artists are still valid
    let top_artists = if now - last_request_top_artists > chrono::Duration::days(1) {
        let top_artist = request::<TopArtists>("https://api.spotify.com/v1/me/top/artists?time_range=medium_term".to_string(), &auth_token, &client).await.expect("Error getting top artists");
        config.set("top_artist".to_string(), Value::TopArtists(top_artist.clone()))
            .write().expect("Error writing config");

        top_artist
    } else {
        match config.get("top_artists") {
            Some(top_artists) => {
                top_artists.get_top_artists().expect("Error getting top artists").clone()
            }
            None => {
                let top_artist = request::<TopArtists>("https://api.spotify.com/v1/me/top/artists?time_range=medium_term".to_string(), &auth_token, &client).await.expect("Error getting top artists");
                config.set("top_artist".to_string(), Value::TopArtists(top_artist.clone()))
                    .write().expect("Error writing config");

                top_artist
            }
        }
    };

    Ok(ProfileData {
        user: user_profile,
        top_tracks,
        top_artists
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use std::fs;
    use crate::util::config::Value;

    #[test]
    fn test_get_user_profile() {
        let filename = format!("test-{}", Uuid::new_v4());
        let user = SpotifyUser::default();
        let _ = Config::new()
            .set_filename(filename.clone())
            .set("user_profile".to_string(), Value::SpotifyUser(user.clone()))
            .write()
            .expect("Failed to write config");

        let user_profile = get_user_profile(filename.clone());
        assert_eq!(user_profile.unwrap(), user);
        fs::remove_file(format!("src/config/{}.json", filename)).expect("Error removing file");
    }

    #[test]
    fn test_get_user_profile_no_user() {
        let filename = format!("test-{}", Uuid::new_v4());
        let _ = Config::new()
            .set_filename(filename.clone())
            .write()
            .expect("Failed to write config");

        let user_profile = get_user_profile(filename.clone());
        assert_eq!(user_profile.unwrap_err(), BbError::NoUserProfileError);
        fs::remove_file(format!("src/config/{}.json", filename)).expect("Error removing file");
    }
}