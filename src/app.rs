use leptos::*;
use leptos_router::*;
use serde_wasm_bindgen::*;
use shared_lib::{
    shared::profile_data::ProfileData,
    shared::global_context::GlobalContext,
};
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::cell::RefCell;
use web_sys::window;
use url::Url;

use crate::{
    ui_elements::{
        side_nav::SideNav,
        header::Header,
        footer::Footer,
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

#[derive(Serialize, Deserialize)]
struct Code <'a> {
    code: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    let profile_signal = create_rw_signal(GlobalContext {
        profile: ProfileData::default(),
        profile_loaded: false,
        currently_playing: None,
    });

    provide_context(profile_signal);

    spawn_local({
        let profile_signal = profile_signal.clone();
        async move {
            let profile = from_value::<ProfileData>(invoke("get_profile_data", JsValue::NULL).await).unwrap_or_default();
            profile_signal.set(GlobalContext {
                profile,
                profile_loaded: true,
                currently_playing: None,
            });
        }
    });

    view! {
        <div>
            <Router>
                <div class="min-h-screen grid grid-cols-[auto_1fr]">
                    <SideNav/>

                    <div class="grid grid-rows-[auto, 1fr, auto] gap-2 w-full text-2xl font-bold px-2 h-screen">
                        <Header/>

                        <div class="border border-neutral-content rounded px-2 py-2 overflow-y-scroll hide-scroll-bar">
                            <Routes>
                                <Route path="/" ssr=SsrMode::OutOfOrder view=Home/>
                                <Route path="/myLibrary" ssr=SsrMode::OutOfOrder view=MyLibrary/>
                                <Route path="/saved" ssr=SsrMode::OutOfOrder view=Saved/>
                                <Route path="/albums" ssr=SsrMode::OutOfOrder view=Albums/>
                                <Route path="/search" ssr=SsrMode::OutOfOrder view=Search/>
                                <Route path="/artists" ssr=SsrMode::OutOfOrder view=Artists/>
                                <Route path="/audiobooks" ssr=SsrMode::OutOfOrder view=Audiobooks/>
                                <Route path="/folders" ssr=SsrMode::OutOfOrder view=Folders/>
                                <Route path="/podcasts" ssr=SsrMode::OutOfOrder view=Podcasts/>
                                <Route path="/profile" ssr=SsrMode::OutOfOrder view=Profile/>
                            </Routes>

                        </div>

                        <Footer/>
                    </div>
                    <Modal/>
                </div>
            </Router>
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