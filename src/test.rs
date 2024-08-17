use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use leptos_router::*;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;

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
pub fn Main() -> impl IntoView {
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
        <dialog id="my_modal_3" class="modal">
            <div class="modal-box">
                <h3>{"Authorize your Account!"}</h3>

                <form on:submit=authorize>
                    <button type="submit" class="btn btn-primary">Authorize</button>
                </form>
            </div>
        </dialog>
    }
}