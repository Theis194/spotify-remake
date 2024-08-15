use leptos::*;

#[component]
pub fn SideNav() -> impl IntoView {
    view! {
        <div class="sidebar bg-neutral text-neutral-content space-y-6 py-2 px-2 absolute inset-y-0 left-0 transform -translate-x-full md:relative md:translate-x-0 transition duration-200 ease-in-out">
            <nav>
                <a href="/myLibrary" class="flex items-center space-x-2 block py-2.5 px-4 rounded transition duration-200 hover:bg-primary hover:text-primary-content">
                    <div class="colored_library size-6 duration-200"></div>

                    <span>My Library</span>
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
    }
}