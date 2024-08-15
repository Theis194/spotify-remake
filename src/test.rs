use leptos::*;
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
                    <div class="sidebar bg-blue-800 text-blue-100 w-64 space-y-6 py-7 px-2 absolute inset-y-0 left-0 transform -translate-x-full md:relative md:translate-x-0 transition duration-200 ease-in-out">
                        <nav>
                            <a href="/myLibrary" class="flex items-center space-x-2 block py-2.5 px-4 rounded transition duration-200 hover:bg-blue-700 hover:text-white">
                                <svg fill="currentColor" viewBox="0 0 36 36" version="1.1" preserveAspectRatio="xMidYMid meet" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" class="size-6">
                                    <g id="SVGRepo_bgCarrier" stroke-width="0"></g>
                                    <g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g>
                                    <g id="SVGRepo_iconCarrier"> <title>library-line</title> 
                                    <path stroke="currentColor" d="M33.48,29.63,26.74,11.82a2,2,0,0,0-2.58-1.16L21,11.85V8.92A1.92,1.92,0,0,0,19.08,7H14V4.92A1.92,1.92,0,0,0,12.08,3H5A2,2,0,0,0,3,5V32a1,1,0,0,0,1,1H20a1,1,0,0,0,1-1V19.27l5,13.21a1,1,0,0,0,1.29.58l5.61-2.14a1,1,0,0,0,.58-1.29ZM12,8.83V31H5V5h7ZM19,31H14V9h5Zm8.51-.25L21.13,13.92l3.74-1.42,6.39,16.83Z" class="clr-i-outline clr-i-outline-path-1"></path> 
                                    <rect x="0" y="0" width="36" height="36" fill-opacity="0"></rect> </g></svg>
                                <span>My Library</span>
                            </a>
                            <a href="/" class="flex items-center space-x-2 block py-2.5 px-4 rounded transition duration-200 hover:bg-blue-700 hover:text-white">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="m2.25 12 8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25" />
                                </svg>
                                <span>Home</span>
                            </a>
                            <a href="/saved" class="flex items-center space-x-2 block py-2.5 px-4 rounded transition duration-200 hover:bg-blue-700 hover:text-white">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M17.593 3.322c1.1.128 1.907 1.077 1.907 2.185V21L12 17.25 4.5 21V5.507c0-1.108.806-2.057 1.907-2.185a48.507 48.507 0 0 1 11.186 0Z" />
                                </svg>
                                <span>Saved</span>
                            </a>
                            <a href="/albums" class="flex items-center space-x-2 block py-2.5 px-4 rounded transition duration-200 hover:bg-blue-700 hover:text-white">
                                <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" class="size-6">
                                    <circle cx="12" cy="12" r="3" stroke="#1C274C" stroke-width="1.5" stroke="currentColor" class="size-6"/>
                                    <path d="M21.9506 13.0004C21.4489 18.0538 17.1853 22.0004 12 22.0004C6.47715 22.0004 2 17.5233 2 12.0004C2 10.179 2.48697 8.4713 3.33782 7.00043M11 2.0498C9.55385 2.19339 8.19833 2.64506 7 3.33825" stroke="#1C274C" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
                                    <path d="M15 11.9992V2.45703C18.1101 3.43385 20.5654 5.88916 21.5422 8.99923" stroke="#1C274C" stroke-width="1.5" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round"/>
                                </svg>
                                <span>Albums</span>
                            </a>
                        </nav>
                    </div>

                    <div class="flex-1 p-10 text-2xl font-bold">
                        <Routes>
                            <Route path="/" view=Home/>
                            <Route path="/myLibrary" view=myLibrary/>
                            <Route path="/saved" view=Saved/>
                            <Route path="/albums" view=Albums/>
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