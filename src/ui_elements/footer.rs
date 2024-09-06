use leptos::*;
use leptos::logging::log;
use gloo::timers::callback::Interval;
use rust_spotify_web_playback_sdk::prelude as sp;
use shared_lib::shared::{global_context::GlobalContext, track_state::TrackInfo};

use crate::spotify_player::requests::*;

#[component]
pub fn Footer() -> impl IntoView {
    let global_context = expect_context::<RwSignal<GlobalContext>>();

    let (acces_token, set_acces_token) = create_slice(
        global_context,
        |data| data.access_token.clone(),
        |data, value| data.access_token = value,
    );

    let (device_id, set_device_id) = create_slice(
        global_context, 
        |data| data.device_id.clone(), 
        |data, value| data.device_id = value,
    );

    let (is_playing, set_is_playing) = create_slice(
        global_context, 
        |data| data.is_playing.clone(), 
        |data, value| data.is_playing = value,
    );

    let (should_repeat, set_should_repeat) = create_signal(false);
    let (is_shuffling, set_is_shuffling) = create_signal(false);
    let (mute, set_mute) = create_signal(false);
    
    let (track_info, set_track_info) = create_signal(TrackInfo::default());

    let connect = create_action(|_| async {
        match sp::connect().await {
            Ok(_) => log!("Connected to Spotify"),
            Err(e) => log!("Error connecting to Spotify: {:?}", e),
        };
    });

    create_effect(move |_| {
        if acces_token.get().is_empty() {
            log!("No access token");
        } else {
            sp::init(
                move || {
                    acces_token.get().clone()
                }, 
                move || {
                    log!("Player ready");
                    connect.dispatch(());
    
                    let _ = sp::add_listener!("player_state_changed", move |state: sp::StateChange| {
                        let track = state.track_window.current_track;
    
                        let track_info = TrackInfo {
                            artists: track.artists.iter().map(|a| a.name.clone()).collect::<Vec<String>>().join(", "),
                            name: track.name.clone(),
                            album: track.album.name.clone(),
                            duration: track.duration_ms as i32,
                            image: track.album.images.first().unwrap().url.clone(),
                            uri: track.uri.clone(),
                            position: state.position,
                            paused: state.paused,
                            shuffle: state.shuffle,
                            timestamp: state.timestamp,
                        };
    
                        log!("Player state changed: {:?}", track_info.clone());
    
                        set_track_info.set(track_info);
    
                    });
    
                    let _ = sp::add_listener!("ready", move |ready: sp::Player| {
                        let id = ready.device_id.to_string();
                        set_device_id.set(id.clone());
    
                        log!("Player ready: {:?}", id);
                    });
                }, 
                "SpotifyBB", 
                0.5, 
                false,
            );
        }
    });

    let activate_player = create_action(|_| async {
        let _ = sp::activate_element().await;
    });

    let play_pause = move |_| {
        set_is_playing.set(!is_playing.get_untracked());

        if is_playing.get_untracked() {
            spawn_local(async move {
                let _ = play("", &device_id.get_untracked(), &acces_token.get_untracked()).await;
            })
        } else {
            spawn_local(async move {
                let _ = pause(&device_id.get_untracked(), &acces_token.get_untracked()).await;
            })
        }
    };

    let repeat_fn = move |_| {
        set_should_repeat.set(!should_repeat.get_untracked());

        if should_repeat.get_untracked() {
            spawn_local(async move {
                let _ = repeat(&device_id.get_untracked(), &acces_token.get_untracked(), "context").await;
            })
        } else {
            spawn_local(async move {
                let _ = repeat(&device_id.get_untracked(), &acces_token.get_untracked(), "off").await;
            })
        }
    };

    let shuffle_fn = move |_| {
        set_is_shuffling.set(!is_shuffling.get());

        spawn_local(async move {
            let _ = shuffle(&device_id.get(), &acces_token.get(), is_shuffling.get()).await;  
        })
    };

    let skip = move |_| {
        set_is_playing.set(true);
        spawn_local(async move {
            let _ = next(&device_id.get(), &acces_token.get()).await;
        })
    };

    let previous = move |_| {
        set_is_playing.set(true);
        spawn_local(async move {
            let _ = previous(&device_id.get_untracked(), &acces_token.get_untracked()).await;
        })
    };

    let volume_change = move |ev| {
        let audioLevel: u8 = event_target_value(&ev).parse().unwrap();
        
        if audioLevel == 0 {
            set_mute.set(true);
        } else {
            set_mute.set(false);
        }

        spawn_local(async move {
            let _ = volume(&device_id.get_untracked(), &acces_token.get_untracked(), audioLevel).await;
        })
    };

    let update_progress = move || {
        if is_playing.get_untracked() == false {
            return;
        }

        if track_info.get_untracked().timestamp == 0 {
            return;
        }
        let mut info = track_info.get_untracked().clone();
        let current_time = js_sys::Date::now() as i64;
        let elapsed_time = current_time - info.timestamp;

        let new_position = info.position + elapsed_time as i32;

        if new_position > info.duration {
            info.position = info.duration;
        } else {
            info.position = new_position;
        }

        info.timestamp = current_time;

        set_track_info.set(info);
    };

    let _interval = Interval::new(1_000, move || {
        update_progress();
    }).forget();

    view! {
        <footer class="w-full bg-neutral p-2">
            <Transition>
                {move || if acces_token.get().is_empty() {
                } else {
                    activate_player.dispatch(());
                    spawn_local(async move {
                        let last_played = get_last_played_track(&acces_token.get()).await.expect("Error getting last played track");
                        
                        let track_info = TrackInfo {
                            artists: last_played.artists,
                            name: last_played.track_name.clone(),
                            album: "".to_string(),
                            duration: last_played.duration_ms,
                            image: last_played.image_url,
                            uri: "".to_string(),
                            position: 0,
                            paused: false,
                            shuffle: false,
                            timestamp: 0,
                        };
                        
                        set_track_info.set(track_info);
                    })
                }}
            </Transition>
            <div class="grid grid-cols-[auto_1fr_auto] items-center">
                <div class="flex flex-row gap-4 items-center overflow-hidden truncate">
                    <img src={move || {track_info.get().image}} class="rounded w-14 h-14" />
                    <div class="">
                        <h2 class="text-sm -mb-1 text-neutral-content">{
                            move || {
                                track_info.get().name.clone()
                            }
                        }</h2>
                        <h3 class="text-xs text-neutral-content">{
                            move || {
                                track_info.get().artists.clone()
                            }
                        }</h3>
                    </div>
                </div>

                <div class="flex flex-col items-center justify-center w-full gap-2">
                    <div class="flex flex-row items-center gap-4">
                        <div on:click=shuffle_fn class={
                            move || {
                                if is_shuffling.get() {
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
                                    if is_playing.get() {
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
                                if should_repeat.get() {
                                    "colored_loop_activated size-6"
                                } else {
                                    "colored_loop size-6"
                                }
                            }
                        }></div>
                    </div>

                    <div class="flex flex-row items-center justify-center gap-4 w-full">
                        <span class="text-sm">{move || {
                            format_time(track_info.get().position)
                        }}</span>
                        
                        <input type="range" min="0" max=move || {track_info.get().duration.to_string()} value=move || {track_info.get().position.to_string()} class="range range-success range-xs responsive-input" />
                        
                        <span class="text-sm">{move || {
                            format_time(track_info.get().duration)
                        }}</span>
                    </div>
                </div>
                
                <div class="flex flex-row">
                    <div class="dropdown dropdown-top">
                        <label tabindex="0" class="cursor-pointer">
                            <div class=move || {
                            if mute.get() {
                                "colored_mute size-7"
                            } else {
                                "colored_volume size-7"
                            }}></div>
                        </label>
                        <ul tabindex="0" class="dropdown-content pl-2 rounded-box">
                            <li>
                                <div class="h-32 flex justify-center items-center">
                                    <input on:change=volume_change type="range" min="0" max="100" value="50" step="1" orient="vertical" class="h-full"/>
                                </div>
                            </li>
                        </ul>
                    </div>
                    <div class="colored_heart size-7"></div>
                    <div class="colored_devices size-7"></div>
                    <div class="colored_add_playlist size-7"></div>
                    <div class="colored_more size-7"></div>
                    <div class="colored_queue size-7"></div>
                </div>
            </div>
        </footer>
    }
}

fn format_time(time: i32) -> String {
    let min = time / 1000 / 60;
    let sec = (time / 1000) % 60;

    if sec < 10 {
        return format!("{}:0{}", min, sec);
    } else {
        return format!("{}:{}", min, sec);
    }
}