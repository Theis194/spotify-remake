use shared_lib::shared::spotify_objects::user::SpotifyUser;

use crate::util::{
    config::Config,
    spotify_bb_error::BbError
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