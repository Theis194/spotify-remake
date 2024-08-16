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
        <div>
            <h1>Search</h1>
            <h2>{move || format!("Spotify Client ID: {}", spotifyClientID.get())}</h2>
            <h2>{move || format!("Spotify Secret: {}", spotifySecret.get())}</h2>
        </div>
    }
}