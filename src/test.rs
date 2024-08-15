use leptos::*;
use wasm_bindgen::prelude::*;
use leptos_router::*;
use dotenv::dotenv;
use std::env;

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
                                <svg viewBox="0 0 28 28" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" fill="currentColor" >
                                    <g id="SVGRepo_bgCarrier" stroke-width="0"></g>
                                    <g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g>
                                    <g id="SVGRepo_iconCarrier"> <title>ic_fluent_library_28_regular</title> <desc>Created with Sketch.</desc> 
                                    <g id="🔍-Product-Icons" stroke="none" stroke-width="1" fill="none" fill-rule="evenodd"> <g id="ic_fluent_library_28_regular" fill="#212121" fill-rule="nonzero"> 
                                    <path stroke="currentColor" d="M3.9997,3 L5.9897,3 C7.04351818,3 7.9078157,3.81639669 7.98421089,4.85080841 L7.9897,5 L7.9897,23 C7.9897,24.0538182 7.17330331,24.9181157 6.13889159,24.9945109 L5.9897,25 L3.9997,25 C2.94588182,25 2.0815843,24.1836033 2.00518911,23.1491916 L1.9997,23 L1.9997,5 C1.9997,3.94618182 2.81609669,3.0818843 3.85050841,3.00548911 L3.9997,3 L5.9897,3 L3.9997,3 Z M10.9947,3 L12.9897,3 C14.0435182,3 14.9078157,3.81639669 14.9842109,4.85080841 L14.9897,5 L14.9897,23 C14.9897,24.0538182 14.1733033,24.9181157 13.1388916,24.9945109 L12.9897,25 L10.9947,25 C9.93992727,25 9.07649752,24.1836033 9.00018319,23.1491916 L8.9947,23 L8.9947,5 C8.9947,3.94618182 9.81018554,3.0818843 10.8453842,3.00548911 L10.9947,3 L12.9897,3 L10.9947,3 Z M20.1303,5.0264 C20.9735941,5.0264 21.7460232,5.56408858 22.0232306,6.38601897 L22.0693,6.5434 L25.9293,22.0264 C26.1851182,23.0487182 25.6026719,24.0847037 24.6168316,24.4098625 L24.4733,24.4514 L22.5103,24.9404 C22.3483,24.9804 22.1853,25.0004 22.0253,25.0004 C21.1810647,25.0004 20.4094661,24.4618256 20.1323596,23.6406247 L20.0863,23.4834 L16.2253,8.0004 C15.9704364,6.97617273 16.552926,5.94101157 17.5387684,5.61680139 L17.6823,5.5754 L19.6453,5.0864 C19.8073,5.0464 19.9703,5.0264 20.1303,5.0264 Z M5.9897,4.5 L3.9997,4.5 C3.75525556,4.5 3.55031728,4.67777778 3.50779328,4.91042524 L3.4997,5 L3.4997,23 C3.4997,23.2444444 3.67747778,23.4493827 3.91012524,23.4919067 L3.9997,23.5 L5.9897,23.5 C6.23503333,23.5 6.43928025,23.3222222 6.48163964,23.0895748 L6.4897,23 L6.4897,5 C6.4897,4.75555556 6.31271235,4.55061728 6.07953813,4.50809328 L5.9897,4.5 Z M12.9897,4.5 L10.9947,4.5 C10.7493667,4.5 10.5451198,4.67777778 10.5027604,4.91042524 L10.4947,5 L10.4947,23 C10.4947,23.2444444 10.6716877,23.4493827 10.9048619,23.4919067 L10.9947,23.5 L12.9897,23.5 C13.2350333,23.5 13.4392802,23.3222222 13.4816396,23.0895748 L13.4897,23 L13.4897,5 C13.4897,4.75555556 13.3127123,4.55061728 13.0795381,4.50809328 L12.9897,4.5 Z M20.1303,6.5264 L20.0688,6.53015 L20.0688,6.53015 L20.0073,6.5414 L18.0453,7.0304 C17.8070778,7.08995556 17.6518185,7.31148642 17.6673137,7.54750288 L17.6813,7.6364 L21.5413,23.1204 C21.6063,23.3804 21.8383,23.5004 22.0253,23.5004 L22.086675,23.496525 L22.086675,23.496525 L22.1473,23.4844 L24.1103,22.9954 C24.3485222,22.9358444 24.5037815,22.7151037 24.4882863,22.4785605 L24.4743,22.3894 L20.6133,6.9054 C20.5483,6.6444 20.3173,6.5264 20.1303,6.5264 Z" id="🎨-Color"> </path> </g> </g> </g></svg>
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
                            <a href="/search" class="flex items-center space-x-2 block py-2.5 px-4 rounded transition duration-200 hover:bg-blue-700 hover:text-white">
                                <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" class="size-6">
                                    <circle cx="12" cy="12" r="3" stroke="#1C274C" stroke-width="1.5" stroke="currentColor" class="size-6"/>
                                    <path d="M21.9506 13.0004C21.4489 18.0538 17.1853 22.0004 12 22.0004C6.47715 22.0004 2 17.5233 2 12.0004C2 10.179 2.48697 8.4713 3.33782 7.00043M11 2.0498C9.55385 2.19339 8.19833 2.64506 7 3.33825" stroke="#1C274C" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
                                    <path d="M15 11.9992V2.45703C18.1101 3.43385 20.5654 5.88916 21.5422 8.99923" stroke="#1C274C" stroke-width="1.5" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round"/>
                                </svg>
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
    
    
    
    view! {
        <div>
            <h1>Search</h1> 
        </div>
    }
}