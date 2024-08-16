use leptos::*;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn Search() -> impl IntoView {
    let (spotifyClientID, setSpotifyClientID) = create_signal(String::new());
    let (spotifySecret, setSpotifySecret) = create_signal(String::new());

    create_effect(move |_| {
        spawn_local(async move {
            let args = to_value(&()).unwrap();

            let clientID = invoke("getSpotifyClient", args.clone()).await.as_string().unwrap();
            setSpotifyClientID.set(clientID);

            let secret = invoke("getSpotifySecret", args.clone()).await.as_string().unwrap();
            setSpotifySecret.set(secret);
        });
    });

    view! {
        <div class="flex flex-col lg:flex-row gap-4">
            <div>
                <h2>Best Match</h2>
                <div class="flex-none w-full lg:w-96 bg-neutral p-4 rounded">
                    <img src="https://via.placeholder.com/150" alt="Spotify Logo" class="object-cover rounded-full size-24"/>
                    <h2 id="name" class="text-4xl text-neutral-content">name</h2>
                    <h3 id="type" class="text-xl text-neutral-content px-4 bg-base-100 rounded-full w-fit">type</h3>
                </div>
            </div>
                
            <div>
                <h2>Songs</h2>
                <div class="flex-1 w-full p-4">
                    <h2 class="break-all">{move || format!("Spotify Client ID: {}", spotifyClientID.get())}</h2>
                    <h2 class="break-all">{move || format!("Spotify Secret: {}", spotifySecret.get())}</h2>
                </div>
            </div>
        </div>
    }
}