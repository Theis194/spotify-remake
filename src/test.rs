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

                        <a href="/" class="text-white flex items-center space-x-2 px-4">
                        <svg class="w-8 h-8" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4M7.835 4.697a3.42 3.42 0 001.946-.806 3.42 3.42 0 014.438 0 3.42 3.42 0 001.946.806 3.42 3.42 0 013.138 3.138 3.42 3.42 0 00.806 1.946 3.42 3.42 0 010 4.438 3.42 3.42 0 00-.806 1.946 3.42 3.42 0 01-3.138 3.138 3.42 3.42 0 00-1.946.806 3.42 3.42 0 01-4.438 0 3.42 3.42 0 00-1.946-.806 3.42 3.42 0 01-3.138-3.138 3.42 3.42 0 00-.806-1.946 3.42 3.42 0 010-4.438 3.42 3.42 0 00.806-1.946 3.42 3.42 0 013.138-3.138z" />
                        </svg>
                        <span class="text-2xl font-extrabold">Better Dev</span>
                        </a>
                    
                        <nav>
                            <a href="/myLibrary" class="flex items-center space-x-4 block py-2.5 px-4 rounded transition duration-200 hover:bg-blue-700 hover:text-white">
                            <svg class="w-8 h-8" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M9 4.5v15m6-15v15m-10.875 0h15.75c.621 0 1.125-.504 1.125-1.125V5.625c0-.621-.504-1.125-1.125-1.125H4.125C3.504 4.5 3 5.004 3 5.625v12.75c0 .621.504 1.125 1.125 1.125Z" />
                            </svg>
                          
                                My Library
                            </a>
                            <a href="/" class="block py-2.5 px-4 rounded transition duration-200 hover:bg-blue-700 hover:text-white">
                                Home
                            </a>
                            <a href="/test" class="block py-2.5 px-4 rounded transition duration-200 hover:bg-blue-700 hover:text-white">
                                Test
                            </a>
                        </nav>
                    </div>

                    <div class="flex-1 p-10 text-2xl font-bold">
                        <Routes>
                            <Route path="/" view=Home/>
                            <Route path="/test" view=Test/>
                        </Routes>
                    </div>
                </div>
            </Router>
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
fn test() -> impl IntoView {
    view! {
        <div>
            <h1>Test</h1>
        </div>
    }
}