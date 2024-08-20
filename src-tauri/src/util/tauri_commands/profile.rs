use shared_lib::shared::user::SpotifyUser;

use crate::util::{
    config::Config,
    spotify_bb_error::BbError
};

#[tauri::command]
pub fn get_user_profile() -> Result<SpotifyUser, BbError> {
    let config = Config::new()
        .set_filename("cache".to_string())
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