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
        <h1>Search</h1>
        
        <div class="grid grid-cols-1 lg:grid-cols-[32rem_auto] gap-4">
            <div class="bg-primary p-4">
                <h2 class="break-all">{move || format!("Spotify Client ID: {}", spotifyClientID.get())}</h2>
            </div>
        
            <div class="bg-secondary p-4">
                <h2 class="break-all">{move || format!("Spotify Secret: {}", spotifySecret.get())}</h2>
            </div>
        </div>
    }
}