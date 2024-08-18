use wasm_bindgen::prelude::*;
use leptos::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub fn is_authorized() {
    spawn_local(async move {
        let is_user_authorized = invoke("is_user_authorized", JsValue::NULL).await.as_bool().unwrap();
    
        if is_user_authorized {
            let modal = web_sys::window().unwrap()
                .document().unwrap()
                .get_element_by_id("authorize_modal").unwrap();    
            if !modal.class_list().contains("hidden") {
                let _ = modal.class_list().add_1("hidden");
            }
        }
    });
}