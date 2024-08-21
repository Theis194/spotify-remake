use shared_lib::shared::{
    profile_data::ProfileData, 
    spotify_objects::{top_artists::TopArtists, top_tracks::TopTracks, user::SpotifyUser}
};

use crate::util::{
    config::Config,
    spotify_bb_error::BbError,
    spotify::util::request,
};

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
    let config = Config::new()
        .set_filename("cache".to_string())
        .read()
        .expect("Failed to read config");

    let auth_token = match config.get("auth_token") {
        Some(auth_token) => {
            Some(auth_token.get_auth_response().unwrap().access_token.clone())
        }
        None => {
            None
        }
    }.expect("No auth token found");

    let client = reqwest::Client::new();

    let user_profile = match config.get("user_profile") {
        Some(user_profile) => {
            Some(user_profile.get_spotify_user().unwrap())
        }
        None => {
            None
        }
    }.expect("No user profile found").clone();

    let top_tracks = request::<TopTracks>("https://api.spotify.com/v1/me/top/tracks".to_string(), &auth_token, &client).await.expect("Error getting top tracks");
    let top_artists = request::<TopArtists>("https://api.spotify.com/v1/me/top/artists".to_string(), &auth_token, &client).await.expect("Error getting top artists");

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