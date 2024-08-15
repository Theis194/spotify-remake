use ev::MouseEvent;
use leptos::*;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use leptos_router::*;

use crate::{
    ui_elements::{
        side_nav::SideNav,
        header::Header,
    },
    pages::{
        my_library::MyLibrary,
        home::Home,
        saved::Saved,
        albums::Albums,
    },
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}


#[component]
pub fn Main() -> impl IntoView {
    view! {
        <div>
            <Router>
                <div class="relative min-h-screen md:flex">
                    <SideNav/>

                    <div class="flex-1 text-2xl font-bold px-2">
                        <Header/>

                        <div class="flex-1 border border-neutral-content rounded px-2 py-2">
                            <Routes>
                                <Route path="/" view=Home/>
                                <Route path="/myLibrary" view=MyLibrary/>
                                <Route path="/saved" view=Saved/>
                                <Route path="/albums" view=Albums/>
                                <Route path="/search" view=Search/>
                                
                            </Routes>
                        </div>
                    </div>
                </div>
            </Router>
        </div>
    }
}

#[component]
fn search() -> impl IntoView {
    let (spotifyToken, setSpotifyToken) = create_signal(String::new());

    let token = move |ev: MouseEvent| {
        spawn_local(async move {
            let args = to_value(&()).unwrap();

            let token = invoke("getSpotifyToken", args).await.as_string().unwrap();
            setSpotifyToken.set(token);
        });
    };


    view! {
        <div>
            <h1>Search</h1>
            <button on:click=token>Get Spotify Token</button>
            <h2>{move || format!("Spotify token: {}", spotifyToken.get())}</h2>
        </div>
    }
}