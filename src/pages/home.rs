use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use leptos::*;
use web_sys::window;
use url::Url;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct Code <'a> {
    code: &'a str,
}

#[component]
pub fn Home() -> impl IntoView {
    let query_params = get_query_params();

    if let Some(query_params) = query_params {
        for (key, value) in query_params {
            if key == "code" {
                let args = to_value(&Code {
                    code: &value,
                }).unwrap();
                spawn_local(async move {
                    invoke("exchange_code", args).await;
                });
            }
        }
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