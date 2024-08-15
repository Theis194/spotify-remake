use ev::MouseEvent;
use leptos::*;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use leptos_router::*;

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
                <div class="relative min-h-screen md:flex">
                    <div class="sidebar bg-neutral text-neutral-content space-y-6 py-2 px-2 absolute inset-y-0 left-0 transform -translate-x-full md:relative md:translate-x-0 transition duration-200 ease-in-out">
                        <nav>
                            <a href="/myLibrary" class="flex items-center space-x-2 block py-2.5 px-4 rounded transition duration-200 hover:bg-primary hover:text-primary-content">
                                <div class="colored_library size-6 duration-200"></div>

                                <span>My Library</span>
                            </a>

                            <a href="/" class="flex items-center space-x-2 block py-2.5 px-4 rounded transition duration-200 hover:bg-primary hover:text-primary-content">
                                <div class="colored_home size-6 duration-200"></div>

                                <span>Home</span>
                            </a>

                            <a href="/saved" class="flex items-center space-x-2 block py-2.5 px-4 rounded transition duration-200 hover:bg-primary hover:text-primary-content">
                                <div class="colored_saved size-6 duration-200"></div>

                                <span>Saved</span>
                            </a>

                            <a href="/albums" class="flex items-center space-x-2 block py-2.5 px-4 rounded transition duration-200 hover:bg-primary hover:text-primary-content">
                                <div class="colored_albums size-6 duration-200"></div>

                                <span>Albums</span>
                            </a>

                            <a href="/folders" class="flex items-center space-x-2 block py-2.5 px-4 rounded transition duration-200 hover:bg-primary hover:text-primary-content">
                                <div class="colored_folders size-6 duration-200"></div>
                          
                                <span>Folders</span>
                            </a>

                            <a href="/podcasts" class="flex items-center space-x-2 block py-2.5 px-4 rounded transition duration-200 hover:bg-primary hover:text-primary-content">
                                <div class="colored_podcasts size-6 duration-200"></div>

                                <span>Podcasts</span>
                            </a>

                            <a href="/Audiobooks" class="flex items-center space-x-2 block py-2.5 px-4 rounded transition duration-200 hover:bg-primary hover:text-primary-content">
                                <div class="colored_audiobooks size-6 duration-200"></div>

                                <span>Audiobooks</span>
                            </a>

                            <a href="/artists" class="flex items-center space-x-2 block py-2.5 px-4 rounded transition duration-200 hover:bg-primary hover:text-primary-content">
                                <div class="colored_artists size-6 duration-200"></div>
                          
                                <span>Artists</span>
                            </a>
                            <a href="/search" class="flex items-center space-x-2 block py-2.5 px-4 rounded transition duration-200 hover:bg-primary hover:text-primary-content">
                                <div class="colored_search size-6 duration-200"></div>

                                <span>Search</span>
                            </a>
                        </nav>
                    </div>

                    <div class="flex-1 p-10 text-2xl font-bold">
                        <Routes>
                            <Route path="/" view=Home/>
                            <Route path="/myLibrary" view=myLibrary/>
                            <Route path="/saved" view=Saved/>
                            <Route path="/albums" view=Albums/>
                            <Route path="/search" view=Search/>
                            
                        </Routes>
                    </div>
                </div>
            </Router>
        </div>
    }
}

#[component]
fn myLibrary() -> impl IntoView {
    view! {
        <div>
            <h1>My Library</h1>
        </div>
    }
}

#[component]
fn home() -> impl IntoView {
    view! {
        <div>
            <h1>Home</h1>
        </div>
    }
}

#[component]
fn saved() -> impl IntoView {
    view! {
        <div>
            <h1>Saved</h1>
        </div>
    }
}

#[component]
fn albums() -> impl IntoView {
    view! {
        <div>
            <h1>Albums</h1>
        </div>
    }
}

#[component]
fn search() -> impl IntoView {
    let (spotifyToken, setSpotifyToken) = create_signal(String::new());

    let token = move |ev: MouseEvent| {
        spawn_local(async move {
            let args = to_value(&()).unwrap();

            let token = invoke("getSpotifyToken", args).await.as_string().unwrap();
            setSpotifyToken.set(token);
        });
    };


    view! {
        <div>
            <h1>Search</h1>
            <button on:click=token>Get Spotify Token</button>
            <h2>{move || format!("Spotify token: {}", spotifyToken.get())}</h2>
        </div>
    }
}