use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::*;
use wasm_bindgen::prelude::*;
use shared_lib::shared::global_context::GlobalContext;

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

#[derive(Serialize, Deserialize)]
struct UserProfileFilename<'a> {
    filename: &'a str,
}

pub fn Header() -> impl IntoView {
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

    view! {
        <header class="py-2 flex-row">
            <nav class="flex justify-between">
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
                        <input /* on:input=current_search */ type="text" class="w-full rounded-lg p-2 bg-transparent shadow-none focus:bg-neutral" placeholder="Search"/>
                    </a>
                </div>

                <div class="flex items-center">
                    <div class="colored_notifications size-6 px-4"></div>

                    <div class="colored_friends size-6 px-4"></div>
                    
                    <a href="/settings">
                        <div class="colored_settings size-6 px-4"></div>
                    </a>
                    
                    <a href="/profile">
                        <div class="size-6 skeleton rounded-full">
                            <Transition fallback= move ||  {view! {<div class="rounded-full w-32 repsonsive-img skeleton"></div>}}>
                                {move || if is_loading() {
                                    view! {
                                        <img id="profile_pic" alt="Profile picture" src="https://via.placeholder.com/150" tabindex="0" role="button" class="object-cover opacity-0 rounded-full"/>
                                    }
                                } else {
                                    view! {
                                        <img id="profile_pic" alt="Profile picture" src={move || profile_pic()} tabindex="0" role="button" class="object-cover rounded-full"/>
                                    }
                                }}
                            </Transition>
                        </div>
                    </a>
                </div>
            </nav>
        </header>
    }
}