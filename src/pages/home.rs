use serde_wasm_bindgen::to_value;
use shared_lib::shared::global_context::GlobalContext;
use shared_lib::shared::profile_data::ProfileData;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::*;
use wasm_bindgen::prelude::*;
use leptos::*;
use web_sys::window;
use url::Url;
use std::rc::Rc;
use std::cell::RefCell;
use crate::pages::page_util::authorized::is_authorized;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
struct Code <'a> {
    code: &'a str,
}

#[component]
pub fn Home() -> impl IntoView {
    let profile_signal = expect_context::<RwSignal<GlobalContext>>();

    let query_params = get_query_params();
    let spotify_redirect = Rc::new(RefCell::new(false));

    if let Some(query_params) = query_params {
        for (key, value) in query_params {
            if key == "code" {
                let args = to_value(&Code {
                    code: &value,
                }).unwrap();
                let spotify_redirect_clone = Rc::clone(&spotify_redirect);
                spawn_local(async move {
                    *spotify_redirect_clone.borrow_mut() = true;
                    invoke("exchange_code", args).await;
                    is_authorized();
                    fetch_profile_data(profile_signal).await
                });
            }
        }
    }

    if !*spotify_redirect.borrow() {
        is_authorized();
        spawn_local(async move {
            fetch_profile_data(profile_signal).await
        });
    }
    
    
    view! {
        <div class="flex flex-col gap-4">
            <div class="flex flex-row gap-2">
                <button class="btn btn-neutral btn-sm rounded-full">"All"</button>
                <button class="btn btn-neutral btn-sm rounded-full">"Music"</button>
                <button class="btn btn-neutral btn-sm rounded-full">"Podcasts"</button>
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 auto-rows-fr gap-2">
                <div class="w-full aspect-w-3 aspect-h-1 bg-neutral rounded">
                    <img src="https://via.placeholder.com/150" alt="" class="w-1/5 object-cover rounded"/>
                </div>
                <div class="w-full aspect-w-3 aspect-h-1 bg-neutral rounded">
                    <img src="https://via.placeholder.com/150" alt="" class="w-1/5 object-cover rounded"/>
                </div>
                <div class="w-full aspect-w-3 aspect-h-1 bg-neutral rounded">
                    <img src="https://via.placeholder.com/150" alt="" class="w-1/5 object-cover rounded"/>
                </div>
                <div class="w-full aspect-w-3 aspect-h-1 bg-neutral rounded">
                    <img src="https://via.placeholder.com/150" alt="" class="w-1/5 object-cover rounded"/>
                </div>
                <div class="w-full aspect-w-3 aspect-h-1 bg-neutral rounded">
                    <img src="https://via.placeholder.com/150" alt="" class="w-1/5 object-cover rounded"/>
                </div>
                <div class="w-full aspect-w-3 aspect-h-1 bg-neutral rounded">
                    <img src="https://via.placeholder.com/150" alt="" class="w-1/5 object-cover rounded"/>
                </div>
                <div class="w-full aspect-w-3 aspect-h-1 bg-neutral rounded">
                    <img src="https://via.placeholder.com/150" alt="" class="w-1/5 object-cover rounded"/>
                </div>
                <div class="w-full aspect-w-3 aspect-h-1 bg-neutral rounded">
                    <img src="https://via.placeholder.com/150" alt="" class="w-1/5 object-cover rounded"/>
                </div>
            </div>

            <div>
                <h1>"Made for ..."</h1>

                <div class="carousel w-full space-x-4">
                    <div class="carousel-item w-64 p-4 flex flex-col items-center">
                        <img src="image1.jpg" alt="Daily Mix 1" class="w-40 h-40 object-cover rounded-lg mb-4"/>
                        <h3 class="text-lg text-white font-bold">"Daily Mix 1"</h3>
                        <p class="text-sm text-gray-400 truncate max-w-xs">"De Danske Hyrder, Hans Philip, Cyd og..."</p>
                    </div>

                    <div class="carousel-item w-64 p-4 flex flex-col items-center">
                        <img src="image2.jpg" alt="Daily Mix 2" class="w-40 h-40 object-cover rounded-lg mb-4"/>
                        <h3 class="text-lg text-white font-bold">"Daily Mix 2"</h3>
                        <p class="text-sm text-gray-400 truncate max-w-xs">"Sub Zero Project, Refuzion, KELTEK..."</p>
                    </div>

                    <div class="carousel-item w-64 p-4 flex flex-col items-center">
                        <img src="image3.jpg" alt="Daily Mix 3" class="w-40 h-40 object-cover rounded-lg mb-4"/>
                        <h3 class="text-lg text-white font-bold">"Daily Mix 3"</h3>
                        <p class="text-sm text-gray-400 truncate max-w-xs">"Enmity, puremind, Jomarijan og mere..."</p>
                    </div>

                    <div class="carousel-item w-64 p-4 flex flex-col items-center">
                        <img src="image4.jpg" alt="Daily Mix 4" class="w-40 h-40 object-cover rounded-lg mb-4"/>
                        <h3 class="text-lg text-white font-bold">"Daily Mix 4"</h3>
                        <p class="text-sm text-gray-400 truncate max-w-xs">"W&W, Dash Berlin, KAROL G og mere..."</p>
                    </div>

                    <div class="carousel-item w-64 p-4 flex flex-col items-center">
                        <img src="image5.jpg" alt="Daily Mix 5" class="w-40 h-40 object-cover rounded-lg mb-4"/>
                        <h3 class="text-lg text-white font-bold">"Daily Mix 5"</h3>
                        <p class="text-sm text-gray-400 truncate max-w-xs">"puremind, HARDSTYLE DEMON..."</p>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn get_query_params() -> Option<Vec<(String, String)>> {
    let window = window().expect("no global `window` exists");
    let location = window.location();
    let href = location.href().expect("failed to get href");

    let url = Url::parse(&href).expect("failed to parse url");
    Some(url.query_pairs().into_owned().collect())
}

async fn fetch_profile_data(profile_signal: RwSignal<GlobalContext>) {
    let profile = from_value::<ProfileData>(invoke("get_profile_data", JsValue::NULL).await).unwrap_or_default();

    profile_signal.set(GlobalContext {
        profile,
        profile_loaded: true,
        currently_playing: None,
        access_token: profile_signal.get().access_token.clone(),
        device_id: "".to_string(),
        is_playing: false,
    });
}