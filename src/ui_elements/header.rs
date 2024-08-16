use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct Search<'a> {
    current: &'a str,
}

pub fn Header() -> impl IntoView {
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
                    <div class="dropdown dropdown-end size-6">
                        <img alt="Profile picture" src="./public/img/profile_pic.png" tabindex="0" role="button" class="object-cover rounded-full size-6"/>
                        <ul
                          tabindex="0"
                          class="menu dropdown-content bg-base-100 rounded-box z-[1] mt-4 w-52 p-2 shadow">
                            <li><a>Item 1</a></li>
                            <li><a>Item 2</a></li>
                        </ul>
                    </div>
                </div>
            </nav>
        </header>
    }
}