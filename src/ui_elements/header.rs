use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::*;
use wasm_bindgen::prelude::*;
use shared_lib::shared::user::SpotifyUser;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
struct Search<'a> {
    current: &'a str,
}

pub fn Header() -> impl IntoView {
    let (profile_pic, set_profile_pic) = create_signal(String::new());

    spawn_local(async move {
        let user_profile = from_value::<SpotifyUser>(invoke("get_user_profile", to_value(&()).unwrap()).await);
        let url = user_profile.unwrap().images[0].url.clone();
        log(&url);
        set_profile_pic.set(url);
    });

    // This function calls the current_search function in the main.rs file
    // with the current search value
    let current_search = move |ev| {
        let args = to_value(&Search {
            current: &event_target_value(&ev),
        });

        // Spawn_local is needed to be able to run invoke
        spawn_local(async move {
            invoke("current_search", args.unwrap()).await;
        })
    };

    view! {
        <header class="py-2 flex-row">
            <nav class="w-full flex justify-between">
                <div class="flex">
                    <a class="flex items-center px-2 transition duration-200 hover:text-primary" href="/">
                        <div class="colored_home size-6 duration-200">test</div>

                        <span>Home</span>
                    </a>

                    <a class="flex items-center px-2 transition duration-200 hover:text-primary" href="/discovery">
                        <div class="colored_discover size-6 duration-200">test</div>

                        <span>Discovery</span>
                    </a>

                    <a class="flex items-center transition duration-200 hover:text-primary" href="/search">
                        <div class="colored_search size-6 duration-200"></div>
                        <input on:input=current_search type="text" class="w-full rounded-lg p-2 bg-transparent shadow-none focus:bg-neutral" placeholder="Search"/>
                    </a>
                </div>

                <div class="flex items-center">
                    <div class="colored_notifications size-6 px-4"></div>

                    <div class="colored_friends size-6 px-4"></div>
                    
                    <a href="/settings">
                        <div class="colored_settings size-6 px-4"></div>
                    </a>
                    
                    <a href="/profile">
                        <div class="size-6">
                        <img id="profile_pic" alt="Profile picture" src={move || profile_pic.get()} tabindex="0" role="button" class="object-cover rounded-full"/>
                        </div>
                    </a>
                </div>
            </nav>
        </header>
    }
}