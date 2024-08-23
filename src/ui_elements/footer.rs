use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    let (play, set_play) = create_signal(false);

    let play_pause = move |_| {
        set_play.set(!play.get());
    };

    view! {
        <footer class="w-full bg-neutral p-4">
            <div class="flex flex-row items-center gap-4">
                <div class="colored_shuffle size-6"></div>
                <div class="colored_step_back size-3"></div>
                <div class="rounded-full bg-primary">
                    <div on:click=play_pause class={
                        move || {
                            if play.get() {
                                "size-6 m-1 colored_pause"
                            } else {
                                "size-6 m-1 colored_play"
                            }
                        }
                    }></div>
                </div>
                <div class="colored_step_forward size-3"></div>
                <div class="colored_loop size-6"></div>
            </div>
        </footer>
    }
}