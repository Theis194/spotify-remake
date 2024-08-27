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
        <div>
            <h1>Home</h1>
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
        acces_token: profile_signal.get().acces_token.clone()
    });
}