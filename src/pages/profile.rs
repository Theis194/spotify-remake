use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::*;
use wasm_bindgen::prelude::*;
use shared_lib::shared::{
    global_context::GlobalContext,  spotify_objects::{
        artist::Artist, top_artists::TopArtists, top_tracks::TopTracks, track::Track,
    }
};

use crate::spotify_player::requests::play;

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
    let profile_data = expect_context::<RwSignal<GlobalContext>>();

    let (profile_loaded, set_profile_loaded) = create_slice(
        profile_data,
        |data| data.profile_loaded,
        |data, value| data.profile_loaded = value,
    );

    let is_loading = move || {
        !profile_loaded.get()
    };

    let profile_pic = move || {
        match profile_data.try_get() {
            Some(data) => data.profile.user.images.get(1).map_or_else(|| "".to_string(), |img| img.url.clone()),
            None => {
                log("Profile data not available");
                "".to_string()
            }
        }
    };

    let profile = move || {
        match profile_data.try_get() {
            Some(data) => data.profile.user.clone(),
            None => {
                log("Profile data not available");
                // Return a default value or handle the error appropriately
                Default::default()
            }
        }
    };

    let top_tracks = move || {
        match profile_data.try_get() {
            Some(data) => data.profile.top_tracks.clone(),
            None => {
                log("Profile data not available");
                TopTracks::default() // Return an empty vector or handle the error appropriately
            }
        }
    };

    let top_artists = move || {
        match profile_data.try_get() {
            Some(data) => data.profile.top_artists.clone(),
            None => {
                log("Profile data not available");
                TopArtists::default() // Return an empty vector or handle the error appropriately
            }
        }
    };

    view! {
        <div>
            // Profile
            <div class="flex items-end">
                <Transition fallback= move ||  {view! {<div class="rounded-full w-32 repsonsive-img skeleton"></div>}}>
                    {move || if is_loading() {
                        view! {
                            <div class="skeleton rounded-full overflow-hidden">
                                <img src="https://via.placeholder.com/150" alt="profile pic" class="object-cover opacity-0 responsive-img"></img>
                            </div>
                        }
                    } else {
                        view! {
                            <div class="object-scale-down">
                                <img src={move || profile_pic()} alt="profile pic" class="object-cover rounded-full responsive-img"></img>
                            </div>
                        }
                    }}
                </Transition>
                <div class="flex-1 w-full">
                    <p class="text-xs pb-2">profile</p>
                    <h1 class="text-left text-3xl lg:text-4xl xl:text-5xl 2xl:text-6xl">{move || profile().display_name}</h1>
                </div>
            </div>

            // Top artists
            <div id="Popular artists">
                <h2 class="text-2xl">"Popular artists"</h2>
                
                <Transition fallback = move || {view! {<div>"Loading"</div>}}>
                    {move || if is_loading() {
                        view! {
                            <div class="carousel">
                                {move || {
                                    let items = (0..5).map(|_| {
                                        view! {
                                            <div class="carousel-item skeleton w-1/3 md:w-1/4 lg:w-1/5">
                                                <div class="relative aspect-[4/3]">
                                                    <img src="https://via.placeholder.com/300" class="w-full opacity-0 h-full object-cover" />
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Vec<_>>();

                                   items.into_iter().collect::<Vec<_>>() 
                                }}
                            </div>
                        }
                    } else {
                        view! {
                            <div class="carousel">
                                {move || top_artists().items.iter().map(|artist| artist_list_item(artist)).collect::<Vec<_>>()}
                            </div>
                        }
                    }}
                </Transition>
            </div>

            // Top tracks
            <div id="Popular songs overflow-hidden">
                <h2 class="text-2xl">"Popular songs"</h2>

                <Transition fallback = move || {view! {<div>"Loading"</div>}}>
                    {move || if is_loading() {
                        view! {
                            <div class="grid grid-cols-1">
                                {move || {
                                    let items = (0..5).map(|_| {
                                        view! {
                                            <div class="flex justify-between items-center rounded px-2 py-2 skeleton hover:bg-neutral">
                                                <div class="flex w-3/5 items-center">
                                                    <img src="https://via.placeholder.com/300" class="size-8 opacity-0"/>
                                                    <div class="px-4">
                                                        <h2 class="text-base -mb-1 text-neutral-content w-32"></h2>
                                                        <h3 class="text-sm text-neutral-content w-32"></h3>
                                                    </div>
                                                </div>

                                                <div class="flex grow w-2/5 text-neutral-content justify-between gap-10 text-sm">
                                                    <div><p class="w-32"></p></div>
                                                    <div><p class="w-32"></p></div>
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Vec<_>>();

                                    items.into_iter().collect::<Vec<_>>()
                                }}
                            </div>
                        }
                    } else {
                        view! {
                            <div class="grid grid-cols-1">
                                {move || top_tracks().items.iter().map(|track| song_list_item(track)).collect::<Vec<_>>()}
                            </div>
                        }
                    }}
                </Transition>
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

    let song_clone = song.clone();
    let click_song = move |_| {
        let global_context = expect_context::<RwSignal<GlobalContext>>();
        
        let (access_token, set_access_token) = create_slice(
            global_context,
            |data| data.access_token.clone(),
            |data, value| data.access_token = value,
        );

        let (device_id, set_device_id) = create_slice(
            global_context, 
            |data| data.device_id.clone(), 
            |data, value| data.device_id = value,
        );

        let (is_playing, set_is_playing) = create_slice(
            global_context, 
            |data| data.is_playing.clone(), 
            |data, value| data.is_playing = value,
        );

        let song = song_clone.clone();
        spawn_local(async move {
            let _ = play(&song.uri.clone(), &device_id.get().clone(), &access_token.get().clone()).await;
        });

        set_is_playing.set(true);
    };

    view! {
        <div on:click=click_song class="flex justify-between items-center bg-base-100 rounded px-2 py-2 hover:bg-neutral">
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

fn artist_list_item(artist: &Artist) -> impl IntoView {
    let artist_img = artist.images[0].url.clone();
    let artist_name = &artist.name;
    let genres = artist.genres.join(", ");

    view! {
        <div class="carousel-item w-1/3 md:w-1/4 lg:w-1/5">
            <div class="relative aspect-[4/3]">
                <img src={artist_img} class="w-full h-full object-cover" />
                <div class="absolute bottom-0 left-0 p-2 bg-opacity-60 bg-black text-white w-full">
                    <h2 class="text-sm md:text-base lg:text-lg font-bold">{artist_name}</h2>
                    <p class="text-xs md:text-sm lg:text-base mt-2 truncate">{genres}</p>
                </div>
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