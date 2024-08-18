use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use leptos::leptos_dom::helpers::window_event_listener;
use leptos_router::*;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

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
        search::Search,
    },
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div>
            <Router>
                <div class="relative min-h-screen flex">
                    <SideNav/>

                    <div class="flex flex-col w-full text-2xl font-bold px-2 h-screen">
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
                    <Modal/>
                </div>
            </Router>
        </div>
    }
}


fn Modal() -> impl IntoView {
    let authorize = move |ev: SubmitEvent| {
        ev.prevent_default();
        
        spawn_local(async move {
            let auth_url = invoke("authorize", to_value(&()).unwrap()).await.as_string().unwrap();

            web_sys::window().unwrap().location().set_href(&auth_url).unwrap();
        });
    };

    view! {
        <div id="authorize_modal" class="fixed left-0 top-0 flex h-full w-full items-center justify-center bg-black bg-opacity-40 py-10">
            <div class="flex modal-box items-center flex-col gap-4">
                <h1>{"Authorize your account to use the app"}</h1>

                <form on:submit=authorize>
                    <button type="submit" class="btn btn-primary">Authorize</button>
                </form>
            </div>
        </div>
    }
}