pub mod shared {
    pub mod profile_data;
    pub mod global_context;

    // Spotify objects
    pub mod spotify_objects {
        pub mod user;
        pub mod artist;
        pub mod track;
        pub mod album;
        pub mod image;
        pub mod top_tracks;
        pub mod top_artists;
        pub mod episode;
        pub mod show;

        pub mod simplified {
            pub mod artist;
        }
    }

    // Spotify object components
    pub mod spotify_object_components {
        pub mod external_urls;
        pub mod followers;
        pub mod external_ids;
        pub mod restrictions;
        pub mod linked_from;
        pub mod device;
        pub mod context;
        pub mod item;
        pub mod resume_point;
        pub mod copyrights;
        pub mod action;
    }
}