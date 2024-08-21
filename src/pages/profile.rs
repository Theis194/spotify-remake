use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::*;
use wasm_bindgen::prelude::*;
use shared_lib::shared::spotify_objects::user::SpotifyUser;

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

struct Song {
    name: String,
    artist: String,
    album: String,
    duration: i32,
    image_src: String,
}

#[component]
pub fn Profile() -> impl IntoView {
    let (profile, set_profile) = create_signal(SpotifyUser::default());
    let (profile_loaded, set_profile_loaded) = create_signal(false);

    spawn_local(async move {
        let user_profile = from_value::<SpotifyUser>(invoke("get_user_profile", to_value(&UserProfileFilename{filename: "cache"}).unwrap()).await);

        set_profile.set(user_profile.unwrap());
        set_profile_loaded.set(true);
    });

    let profile_pic = move || {
        if profile_loaded.get() {
            return profile.get().images[1].url.clone();
        } else {
            return String::new();
        }
    };

    let songs = vec![
        Song {
            name: "Song 1".to_string(),
            artist: "Artist 1".to_string(),
            album: "Album 1".to_string(),
            duration: 180,
            image_src: "https://via.placeholder.com/150".to_string(),
        },
        Song {
            name: "Song 2".to_string(),
            artist: "Artist 2".to_string(),
            album: "Album 2".to_string(),
            duration: 240,
            image_src: "https://via.placeholder.com/150".to_string(),
        },
        Song {
            name: "Song 3".to_string(),
            artist: "Artist 3".to_string(),
            album: "Album 3".to_string(),
            duration: 300,
            image_src: "https://via.placeholder.com/150".to_string(),
        },
        Song {
            name: "Song 4".to_string(),
            artist: "Artist 4".to_string(),
            album: "Album 4".to_string(),
            duration: 360,
            image_src: "https://via.placeholder.com/150".to_string(),
        },
        Song {
            name: "Song 5".to_string(),
            artist: "Artist 5".to_string(),
            album: "Album 5".to_string(),
            duration: 420,
            image_src: "https://via.placeholder.com/150".to_string(),
        },
    ];

    view! {
        <div>
            <div class="flex items-end">
                <div class="object-scale-down">
                    <img src={move || profile_pic} alt="profile pic" class="object-cover rounded-full responsive-img"></img>
                </div>
                <div class="flex-1 w-full">
                    <p class="text-xs pb-2">profile</p>
                    <h1 class="text-left text-3xl lg:text-4xl xl:text-5xl 2xl:text-6xl">{move || profile.get().display_name}</h1>
                </div>
            </div>

            <div>
            
            </div>

            <div id="Popular songs">
                <h2 class="text-2xl">Popular songs</h2>
                <div class="grid grid-cols-1">
                    {songs.into_iter()
                        .map(|n| song_list_item(n))
                        .collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}

fn song_list_item(song: Song) -> impl IntoView {
    view! {
        <div class="flex justify-between items-center bg-base-100 rounded px-2 py-2 hover:bg-neutral">
            <div class="flex w-3/5 items-center">
                <img src={song.image_src} class="size-8"/>
                <div class="px-4">
                    <h2 class="text-base -mb-1 text-neutral-content">{song.name}</h2>
                    <h3 class="text-sm text-neutral-content">{song.artist}</h3>
                </div>
            </div>

            <div class="flex grow w-2/5 text-neutral-content justify-between gap-10 text-sm">
                <div><p>{song.album}</p></div>
                <div><p>{format_time(song.duration)}</p></div>
            </div>
        </div>
    }
}

fn format_time(time: i32) -> String {
    let min = time / 60;
    let sec = time % 60;

    if sec < 10 {
        return format!("{}:0{}", min, sec)
    }

    format!("{}:{}", min, sec)
}