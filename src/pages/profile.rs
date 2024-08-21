use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::*;
use wasm_bindgen::prelude::*;
use shared_lib::shared::{
    profile_data::ProfileData, 
    spotify_objects::{
        top_artists::TopArtists, 
        top_tracks::TopTracks, 
        track::Track, 
        user::SpotifyUser
    }
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
struct UserProfileFilename<'a> {
    filename: &'a str,
}

#[component]
pub fn Profile() -> impl IntoView {
    let (profile_data, set_profile_data) = create_signal(ProfileData::default());
    let (profile_data_loaded, set_profile_data_loaded) = create_signal(false);

    spawn_local(async move {
        let profile_data = from_value::<ProfileData>(invoke("get_profile_data", JsValue::NULL).await);
        
        set_profile_data.set(profile_data.unwrap());
        set_profile_data_loaded.set(true);
    });

    let profile_pic = move || {
        if profile_data_loaded.get() {
            return profile_data.get().user.images[1].url.clone();
        } else {
            return String::new();
        }
    };

    let profile = move || {
        if profile_data_loaded.get() {
            return profile_data.get().user.clone();
        } else {
            return SpotifyUser::default();
        }
    };

    let mut top_tracks = move || {
        if profile_data_loaded.get() {
            return profile_data.get().top_tracks.clone();
        } else {
            return TopTracks::default();
        }
    };

    let top_artists = move || {
        if profile_data_loaded.get() {
            return profile_data.get().top_artists.clone();
        } else {
            return TopArtists::default();
        }
    };

    view! {
        <div>
            <div class="flex items-end">
                <div class="object-scale-down">
                    <img src={move || profile_pic} alt="profile pic" class="object-cover rounded-full responsive-img"></img>
                </div>
                <div class="flex-1 w-full">
                    <p class="text-xs pb-2">profile</p>
                    <h1 class="text-left text-3xl lg:text-4xl xl:text-5xl 2xl:text-6xl">{move || profile().display_name}</h1>
                </div>
            </div>

            <div>
            
            </div>

            <div id="Popular songs">
                <h2 class="text-2xl">Popular songs</h2>
                <div class="grid grid-cols-1">
                    {move || top_tracks().items.iter().map(|track| song_list_item(track)).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}

fn song_list_item(song: &Track) -> impl IntoView {
    let album_img = song.album.images[0].url.clone();
    let song_name = &song.name;
    let artist_names = song.artists.iter().map(|artist| artist.name.clone()).collect::<Vec<String>>().join(", ");
    let album_name = song.album.name.clone();
    let duration = format_time(song.duration_ms);

    view! {
        <div class="flex justify-between items-center bg-base-100 rounded px-2 py-2 hover:bg-neutral">
            <div class="flex w-3/5 items-center">
                <img src={album_img} class="size-8"/>
                <div class="px-4">
                    <h2 class="text-base -mb-1 text-neutral-content">{song_name}</h2>
                    <h3 class="text-sm text-neutral-content">{artist_names}</h3>
                </div>
            </div>

            <div class="flex grow w-2/5 text-neutral-content justify-between gap-10 text-sm">
                <div><p>{album_name}</p></div>
                <div><p>{duration}</p></div>
            </div>
        </div>
    }
}

fn format_time(time: i32) -> String {
    let min = time / 1000 / 60;
    let sec = (time / 1000) % 60;

    if sec < 10 {
        return format!("{}:0{}", min, sec);
    } else {
        return format!("{}:{}", min, sec);
    }
}