use leptos::*;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

struct Song {
    name: String,
    artist: String,
    duration: i32,
    image_src: String,
}

#[component]
pub fn Search() -> impl IntoView {
    let (spotifyClientID, setSpotifyClientID) = create_signal(String::new());
    let (spotifySecret, setSpotifySecret) = create_signal(String::new());

    create_effect(move |_| {
        spawn_local(async move {
            // let args = to_value(&()).unwrap();

        });
    });

    let songs = vec![
        Song {
            name: "Song 1".to_string(),
            artist: "Artist 1".to_string(),
            duration: 180,
            image_src: "https://via.placeholder.com/150".to_string(),
        },
        Song {
            name: "Song 2".to_string(),
            artist: "Artist 2".to_string(),
            duration: 240,
            image_src: "https://via.placeholder.com/150".to_string(),
        },
        Song {
            name: "Song 3".to_string(),
            artist: "Artist 3".to_string(),
            duration: 300,
            image_src: "https://via.placeholder.com/150".to_string(),
        },
        Song {
            name: "Song 4".to_string(),
            artist: "Artist 5".to_string(),
            duration: 360,
            image_src: "https://via.placeholder.com/150".to_string(),
        },
    ];

    view! {
        <div class="flex flex-col lg:flex-row gap-4">
            <div>
                <h2 class="pb-2">Best Match</h2>
                <div class="flex-none w-96 bg-neutral p-4 rounded">
                    <img src="https://via.placeholder.com/150" alt="Spotify Logo" class="object-cover rounded-full size-24"/>
                    <h2 id="name" class="text-4xl text-neutral-content pt-4">name</h2>
                    <h3 id="type" class="text-xl text-neutral-content px-4 bg-base-100 rounded-full w-fit">type</h3>
                </div>
            </div>
                
            <div>
                <h2>Songs</h2>
                
                <div>
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
            <div class="flex items-center">
                <img src={song.image_src} class="size-8"/>
                <div class="px-4">
                    <h2 class="text-base -mb-1 text-neutral-content">{song.name}</h2>
                    <h3 class="text-sm text-neutral-content">{song.artist}</h3>
                </div>
            </div>

            <div class="flex text-neutral-content">
                <p>{format_time(song.duration)}</p>
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