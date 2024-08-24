use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    let (play, set_play) = create_signal(false);

    let play_pause = move |_| {
        set_play.set(!play.get());
    };

    view! {
        <footer class="w-full bg-neutral p-2">
            <div class="grid grid-cols-[auto_1fr_auto] items-center">
                <div class="flex flex-row gap-4 items-center">
                    <img src="https://via.placeholder.com/50" class="rounded w-14 h-14" />
                    <div>
                        <h2 class="text-sm -mb-1 text-neutral-content">"{song.name}"</h2>
                        <h3 class="text-xs text-neutral-content">"{song.artist}"</h3>
                    </div>
                </div>

                <div class="flex flex-col items-center justify-center w-full gap-2">
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

                    <div class="flex flex-row items-center justify-center gap-4 w-full">
                        <span class="text-sm">"0:00"</span>
                        <input type="range" min="0" max="100" value="40" class="range range-success range-xs responsive-input" />
                        <span class="text-sm">"0:00"</span>
                    </div>
                </div>
                
                <div class="flex flex-row">
                    <div class="colored_shuffle size-6"></div>
                    <div class="colored_shuffle size-6"></div>
                    <div class="colored_shuffle size-6"></div>
                    <div class="colored_shuffle size-6"></div>
                    <div class="colored_shuffle size-6"></div>
                </div>
            </div>
        </footer>
    }
}