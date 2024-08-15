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
                    <div class="sidebar bg-blue-800 text-blue-100 space-y-6 py-2 px-2 absolute inset-y-0 left-0 transform -translate-x-full md:relative md:translate-x-0 transition duration-200 ease-in-out">
                        <nav>
                            <a href="/myLibrary" class="flex items-center space-x-2 block py-2.5 px-4 rounded transition duration-200 hover:bg-blue-700 hover:text-white">
                                <img src="../img/library.svg"/>
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

                            <a href="/folders" class="flex items-center space-x-2 block py-2.5 px-4 rounded transition duration-200 hover:bg-blue-700 hover:text-white">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 12.75V12A2.25 2.25 0 0 1 4.5 9.75h15A2.25 2.25 0 0 1 21.75 12v.75m-8.69-6.44-2.12-2.12a1.5 1.5 0 0 0-1.061-.44H4.5A2.25 2.25 0 0 0 2.25 6v12a2.25 2.25 0 0 0 2.25 2.25h15A2.25 2.25 0 0 0 21.75 18V9a2.25 2.25 0 0 0-2.25-2.25h-5.379a1.5 1.5 0 0 1-1.06-.44Z" />
                                </svg>
                          
                                <span>Folders</span>
                            </a>

                            <a href="/podcasts" class="flex items-center space-x-2 block py-2.5 px-4 rounded transition duration-200 hover:bg-blue-700 hover:text-white">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 18.75a6 6 0 0 0 6-6v-1.5m-6 7.5a6 6 0 0 1-6-6v-1.5m6 7.5v3.75m-3.75 0h7.5M12 15.75a3 3 0 0 1-3-3V4.5a3 3 0 1 1 6 0v8.25a3 3 0 0 1-3 3Z" />
                                </svg>

                                <span>Podcasts</span>
                            </a>

                            <a href="/Audiobooks" class="flex items-center space-x-2 block py-2.5 px-4 rounded transition duration-200 hover:bg-blue-700 hover:text-white">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 6.042A8.967 8.967 0 0 0 6 3.75c-1.052 0-2.062.18-3 .512v14.25A8.987 8.987 0 0 1 6 18c2.305 0 4.408.867 6 2.292m0-14.25a8.966 8.966 0 0 1 6-2.292c1.052 0 2.062.18 3 .512v14.25A8.987 8.987 0 0 0 18 18a8.967 8.967 0 0 0-6 2.292m0-14.25v14.25" />
                                </svg>
                          

                                <span>Audiobooks</span>
                            </a>

                            <a href="/artists" class="flex items-center space-x-2 block py-2.5 px-4 rounded transition duration-200 hover:bg-blue-700 hover:text-white">
                                <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" stroke="currentColor" class="size-6">
                                <g id="SVGRepo_bgCarrier" stroke-width="0"></g>
                                <g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g>
                                <g id="SVGRepo_iconCarrier"> <g clip-path="url(#clip0_429_11111)"> 
                                <circle  cx="12" cy="7" r="3" stroke="currentColor" stroke-width="1.5"></circle> 
                                <circle cx="18" cy="18" r="2" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></circle> 
                                <path stroke="currentColor" d="M12.3414 20H6C4.89543 20 4 19.1046 4 18C4 15.7909 5.79086 14 8 14H13.5278" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path> 
                                <path stroke="currentColor" d="M20 18V11L22 13" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path> </g> 
                                <defs> <clipPath id="clip0_429_11111"> 
                                <rect width="24" height="24" fill="white"></rect> </clipPath> </defs> </g></svg>
                          
                                <span>Artists</span>
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