use leptos::*;
use leptos_router::*;

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
                </div>
            </Router>
        </div>
    }
}
