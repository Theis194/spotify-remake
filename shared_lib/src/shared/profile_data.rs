use serde::{Serialize, Deserialize};

use super::spotify_objects::{
    top_artists::TopArtists, 
    top_tracks::TopTracks, 
    user::SpotifyUser
};

// Spotify User object
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]

pub struct ProfileData {
    pub user: SpotifyUser,
    pub top_tracks: TopTracks,
    pub top_artists: TopArtists,
}