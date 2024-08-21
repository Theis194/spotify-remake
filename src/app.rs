use leptos::*;
use leptos_router::*;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

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
        audiobooks::Audiobooks,
        artists::Artists,
        folders::Folders,
        podcasts::Podcasts,
        profile::Profile,
        page_util::authorized::is_authorized,
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

                        <div class="flex-1 border border-neutral-content rounded px-2 py-2 overflow-y-scroll">
                            <Routes>
                                <Route path="/" view=Home/>
                                <Route path="/myLibrary" view=MyLibrary/>
                                <Route path="/saved" view=Saved/>
                                <Route path="/albums" view=Albums/>
                                <Route path="/search" view=Search/>
                                <Route path="/artists" view=Artists/>
                                <Route path="/audiobooks" view=Audiobooks/>
                                <Route path="/folders" view=Folders/>
                                <Route path="/podcasts" view=Podcasts/>
                                <Route path="/profile" view=Profile/>
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
    let (spotify_url, set_spotify_url) = create_signal(String::new());
    let spotify_redirect = Rc::new(RefCell::new(false));
    let spotify_redirect_clone = Rc::clone(&spotify_redirect);

    spawn_local(async move {
        *spotify_redirect_clone.borrow_mut() = true;
        let url = invoke("authorize", to_value(&()).unwrap()).await.as_string().unwrap();
        set_spotify_url.set(url);
        is_authorized()
    });

    if !*spotify_redirect.borrow() {
        is_authorized();
    }

    view! {
        <div id="authorize_modal" class="fixed left-0 top-0 flex h-full w-full items-center justify-center bg-black bg-opacity-40 py-10">
            <div class="flex modal-box items-center flex-col gap-4">
                <h1>{"Authorize your account to use the app"}</h1>

                <a class="btn btn-primary hover:text-primary-content" href={move || spotify_url.get()}>Authorize</a>
            </div>
        </div>
    }
}