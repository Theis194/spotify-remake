use leptos::*;
use shared_lib::shared::global_context::GlobalContext;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn playMusic(spotify_uri: &str, device_id: &str, acces_token: &str);

    fn getDeviceId() -> JsValue;

    fn pauseMusic(device_id: &str, acces_token: &str);

    fn nextMusic(device_id: &str, acces_token: &str);

    fn previousMusic(device_id: &str, acces_token: &str);

    fn shuffleMusic(device_id: &str, acces_token: &str, shuffle: bool);

    fn repeatMusic(device_id: &str, acces_token: &str, state: &str);

    fn volumeMusic(device_id: &str, acces_token: &str, volume: u8);
}

#[component]
pub fn Footer() -> impl IntoView {
    let global_context = expect_context::<RwSignal<GlobalContext>>();

    let (play, set_play) = create_signal(false);
    let (repeat, set_repeat) = create_signal(false);
    let (shuffle, set_shuffle) = create_signal(false);

    let acces_token = move || {
        match global_context.try_get() {
            Some(data) => data.acces_token.clone(),
            None => {
                "".to_string()
            }
        }
    };

    let device_id = move || {
        match get_spotify_device_id() {
            Some(data) => data,
            None => {
                "".to_string()
            }
        }
    };

    let play_pause = move |_| {
        set_play.set(!play.get());

        if play.get() {
            playMusic("", &device_id(), &acces_token());
        } else {
            pauseMusic(&device_id(), &acces_token());
        }
    };

    let repeat_fn = move |_| {
        set_repeat.set(!repeat.get());

        if repeat.get() {
            repeatMusic(&device_id(), &acces_token(), "context");
        } else {
            repeatMusic(&device_id(), &acces_token(), "off");
        }
    };

    let shuffle_fn = move |_| {
        set_shuffle.set(!shuffle.get());

        shuffleMusic(&device_id(), &acces_token(), shuffle.get());
    };

    let skip = move |_| {
        nextMusic(&device_id(), &acces_token());
    };

    let previous = move |_| {
        previousMusic(&device_id(), &acces_token());
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
                        <div on:click=shuffle_fn class={
                            move || {
                                if shuffle.get() {
                                    "colored_shuffle_activated size-6"
                                } else {
                                    "colored_shuffle size-6"
                                }
                            }
                        }></div>
                        <div on:click=previous class="colored_step_back size-3"></div>
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
                        <div on:click=skip class="colored_step_forward size-3"></div>
                        <div on:click=repeat_fn class={
                            move || {
                                if repeat.get() {
                                    "colored_loop_activated size-6"
                                } else {
                                    "colored_loop size-6"
                                }
                            }
                        }></div>
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

fn get_spotify_device_id() -> Option<String> {
    let device_id = getDeviceId();

    // Convert `JsValue` to a `String`, if it exists
    device_id.as_string()
}