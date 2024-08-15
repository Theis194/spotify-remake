use leptos::*;

pub fn Header() -> impl IntoView {
    view! {
        <header class="py-2">
            <nav class="flex flex-row">
                <a class="flex items-center px-2 transition duration-200 hover:text-primary" href="/">
                    <div class="colored_home size-6 duration-200">test</div>

                    <span>Home</span>
                </a>

                <a class="flex items-center px-2 transition duration-200 hover:text-primary" href="/discovery">
                    <div class="colored_discover size-6 duration-200">test</div>

                    <span>Discovery</span>
                </a>

                <a class="flex items-center transition duration-200 hover:text-primary">
                    <div class="colored_search size-6 duration-200"></div>
                    <input type="text" class="w-full rounded-lg p-2 bg-transparent" placeholder="Search"/>
                </a>
            </nav>
        </header>
    }
}